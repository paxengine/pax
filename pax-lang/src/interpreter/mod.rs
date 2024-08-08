use computable::Computable;
use pax_runtime_api::{pax_value::functions::call_function, PaxValue, Percent, Rotation, Size};
use pax_runtime_api::{CoercionRules, Functions, Numeric};
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::{self, PrattParser},
};
use property_resolution::IdentifierResolver;
use std::{collections::HashMap, rc::Rc};

use crate::{
    deserializer::from_pax_ast, get_pax_pratt_parser, parse_pax_err, parse_pax_pairs, PaxParser,
    Rule,
};

pub(crate) mod computable;
pub mod property_resolution;
mod tests;

#[derive(PartialEq, Debug)]
pub enum PaxExpression {
    Primary(Box<PaxPrimary>),
    Prefix(Box<PaxPrefix>),
    Infix(Box<PaxInfix>),
    Postfix(Box<PaxPostfix>),
}

#[derive(PartialEq, Debug)]
pub enum PaxPrimary {
    Literal(PaxValue),
    Grouped(Box<PaxExpression>, Option<PaxUnit>),
    Identifier(PaxIdentifier, Vec<PaxAccessor>),
    FunctionCall(PaxFunctionCall),
    Object(HashMap<String, PaxExpression>),
    Enum(String, Vec<PaxExpression>),
    Range(PaxExpression, PaxExpression),
    Tuple(Vec<PaxExpression>),
    List(Vec<PaxExpression>),
}

#[derive(PartialEq, Debug)]
pub enum PaxUnit {
    Percent,
    Pixels,
    Radians,
    Degrees,
}

#[derive(PartialEq, Debug)]
pub enum PaxAccessor {
    Tuple(usize),
    List(PaxExpression),
    Struct(String),
}

#[derive(PartialEq, Debug)]
pub struct PaxPrefix {
    operator: PaxOperator,
    rhs: Box<PaxExpression>,
}

#[derive(PartialEq, Debug)]
pub struct PaxInfix {
    operator: PaxOperator,
    lhs: Box<PaxExpression>,
    rhs: Box<PaxExpression>,
}

#[derive(PartialEq, Debug)]
pub struct PaxPostfix {
    operator: PaxOperator,
    lhs: Box<PaxExpression>,
}

#[derive(PartialEq, Debug)]
pub struct PaxOperator {
    name: String,
}

#[derive(PartialEq, Debug)]
pub struct PaxIdentifier {
    name: String,
}

#[derive(PartialEq, Debug)]
pub struct PaxFunctionCall {
    scope: String,
    function_name: String,
    args: Vec<PaxExpression>,
}

/// Parse a pax expression into a computable AST
pub fn parse_pax_expression(
    expr: &str,
    idr: Rc<dyn IdentifierResolver>,
) -> Result<PaxExpression, String> {
    let parsed_expr = parse_pax_pairs(Rule::expression_body, expr)
        .map_err(|e| format!("Failed to parse expression: {}", e))?;
    let pratt_parser = get_pax_pratt_parser();
    recurse_pratt_parse(parsed_expr, &pratt_parser, idr)
}

fn recurse_pratt_parse(
    expr: Pairs<Rule>,
    pratt_parser: &PrattParser<Rule>,
    idr: Rc<dyn IdentifierResolver>,
) -> Result<PaxExpression, String> {
    pratt_parser
        .map_primary(move |primary| match primary.as_rule() {
            Rule::xo_literal => {
                let inner = primary.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::literal_value => {
                        let pax_value = from_pax_ast(inner)
                            .map_err(|e| format!("Failed to parse literal value: {}", e))?;
                        let value = PaxPrimary::Literal(pax_value);
                        let exp = PaxExpression::Primary(Box::new(value));
                        Ok(exp)
                    }
                    Rule::literal_tuple_access => {
                        let mut inner = inner.into_inner();
                        let ident = inner.next().unwrap().as_str().trim().to_string();
                        let index = inner.next().unwrap().as_str().parse::<usize>().unwrap();
                        let value = PaxPrimary::Identifier(
                            PaxIdentifier { name: ident },
                            vec![PaxAccessor::Tuple(index)],
                        );
                        let exp = PaxExpression::Primary(Box::new(value));
                        Ok(exp)
                    }
                    Rule::literal_list_access => {
                        let mut inner = inner.into_inner();
                        let ident = inner.next().unwrap().as_str().trim().to_string();
                        let index = inner.next().unwrap().as_str();
                        let index = parse_pax_expression(index, idr.clone())?;
                        let value = PaxPrimary::Identifier(
                            PaxIdentifier { name: ident },
                            vec![PaxAccessor::List(index)],
                        );
                        let exp = PaxExpression::Primary(Box::new(value));
                        Ok(exp)
                    }
                    _ => {
                        return Err(format!("Unexpected rule: {:?}", inner.as_rule()));
                    }
                }
            }
            Rule::expression_body => {
                recurse_pratt_parse(primary.into_inner(), pratt_parser, idr.clone())
            }
            Rule::expression_grouped => {
                let mut inner = primary.clone().into_inner();
                let expr = inner.next().unwrap();
                let expr_val = recurse_pratt_parse(expr.into_inner(), pratt_parser, idr.clone())?;
                let ret: Result<PaxExpression, String> = if let Some(unit) = inner.next() {
                    let unit = unit.as_str().trim();
                    match unit {
                        "%" => Ok(PaxExpression::Primary(Box::new(PaxPrimary::Grouped(
                            Box::new(expr_val),
                            Some(PaxUnit::Percent),
                        )))),
                        "px" => Ok(PaxExpression::Primary(Box::new(PaxPrimary::Grouped(
                            Box::new(expr_val),
                            Some(PaxUnit::Pixels),
                        )))),
                        "rad" => Ok(PaxExpression::Primary(Box::new(PaxPrimary::Grouped(
                            Box::new(expr_val),
                            Some(PaxUnit::Radians),
                        )))),
                        "deg" => Ok(PaxExpression::Primary(Box::new(PaxPrimary::Grouped(
                            Box::new(expr_val),
                            Some(PaxUnit::Degrees),
                        )))),
                        _ => Err(format!("Unsupported unit: {}", unit)),
                    }
                } else {
                    Ok(PaxExpression::Primary(Box::new(PaxPrimary::Grouped(
                        Box::new(expr_val),
                        None,
                    ))))
                };
                ret
            }
            Rule::xo_enum_or_function_call => {
                let mut inner = primary.into_inner();
                while inner.len() > 3 {
                    inner.next();
                }
                let scope = inner.next().unwrap().as_str().trim().to_string();
                let function_name = inner.next().unwrap().as_str().trim().to_string();

                let args = if let Some(args) = inner.next() {
                    args.into_inner()
                        .map(|a| recurse_pratt_parse(a.into_inner(), pratt_parser, idr.clone()))
                        .collect::<Result<Vec<PaxExpression>, String>>()?
                } else {
                    vec![]
                };

                if Functions::has_function(&scope, &function_name) {
                    let value = PaxFunctionCall {
                        scope,
                        function_name,
                        args,
                    };
                    let exp = PaxExpression::Primary(Box::new(PaxPrimary::FunctionCall(value)));
                    Ok(exp)
                } else {
                    let exp =
                        PaxExpression::Primary(Box::new(PaxPrimary::Enum(function_name, args)));
                    Ok(exp)
                }
            }
            Rule::xo_color_space_func => {
                let func = primary.as_str().trim().split("(").next().unwrap();
                let inner = primary.into_inner();
                let args = inner
                    .map(|a| recurse_pratt_parse(a.into_inner(), pratt_parser, idr.clone()))
                    .collect::<Result<Vec<PaxExpression>, String>>()?;
                let value = PaxFunctionCall {
                    scope: "Color".to_string(),
                    function_name: func.to_string(),
                    args,
                };
                let exp = PaxExpression::Primary(Box::new(PaxPrimary::FunctionCall(value)));
                Ok(exp)
            }
            Rule::xo_object => {
                let mut inner = primary.into_inner();
                while inner.peek().unwrap().as_rule() == Rule::identifier {
                    inner.next();
                }
                let mut obj = HashMap::new();
                for pair in inner {
                    let mut pair = pair.into_inner();
                    // settings_key = { identifier ~ (":" | "=") }
                    let key = pair
                        .next()
                        .unwrap()
                        .as_str()
                        .trim()
                        .trim_end_matches(':')
                        .trim_end_matches('=')
                        .to_string();
                    let value = pair.next().unwrap();
                    let value = recurse_pratt_parse(value.into_inner(), pratt_parser, idr.clone())?;
                    obj.insert(key, value);
                }
                let value = PaxPrimary::Object(obj);
                let exp = PaxExpression::Primary(Box::new(value));
                Ok(exp)
            }
            Rule::xo_range => {
                let mut inner = primary.into_inner();
                let start_rule = Pairs::single(inner.next().unwrap());
                let start = recurse_pratt_parse(start_rule, pratt_parser, idr.clone())?;
                // xo_range_exclusive
                inner.next();
                let end_rule = Pairs::single(inner.next().unwrap());
                let end = recurse_pratt_parse(end_rule, pratt_parser, idr.clone())?;
                let value = PaxPrimary::Range(start, end);
                let exp = PaxExpression::Primary(Box::new(value));
                Ok(exp)
            }
            Rule::xo_tuple => {
                let inner = primary.into_inner();
                let tuple = inner
                    .map(|e| recurse_pratt_parse(e.into_inner(), pratt_parser, idr.clone()))
                    .collect::<Result<Vec<PaxExpression>, String>>()?;
                let value = PaxPrimary::Tuple(tuple);
                let exp = PaxExpression::Primary(Box::new(value));
                Ok(exp)
            }
            Rule::xo_list => {
                let inner = primary.into_inner();
                let list = inner
                    .map(|e| recurse_pratt_parse(e.into_inner(), pratt_parser, idr.clone()))
                    .collect::<Result<Vec<PaxExpression>, String>>()?;
                let value = PaxPrimary::List(list);
                let exp = PaxExpression::Primary(Box::new(value));
                Ok(exp)
            }
            Rule::xo_symbol => {
                let mut symbols = primary.into_inner();

                // skip self or this
                let peek = symbols.peek().unwrap();
                if peek.as_str().trim() == "self" || peek.as_str().trim() == "this" {
                    symbols.next();
                }
                let identifier = symbols.next().unwrap().as_str().trim().to_string();
                let mut accessors = vec![];
                for symbol in symbols {
                    let accessor = match symbol.as_rule() {
                        // list access
                        Rule::expression_body => {
                            let expr = recurse_pratt_parse(
                                symbol.into_inner(),
                                pratt_parser,
                                idr.clone(),
                            )?;
                            PaxAccessor::List(expr)
                        }
                        // field access .field
                        Rule::identifier => {
                            let field = symbol.as_str().trim().to_string();
                            PaxAccessor::Struct(field)
                        }
                        _ => {
                            return Err(format!("Unexpected rule: {:?}", symbol.as_rule()));
                        }
                    };
                    accessors.push(accessor);
                }
                let value = PaxPrimary::Identifier(PaxIdentifier { name: identifier }, accessors);
                let exp = PaxExpression::Primary(Box::new(value));
                Ok(exp)
            }
            _ => {
                return Err(format!(
                    "Unexpected rule: {:?}, str: {} ",
                    primary.as_rule(),
                    primary.as_str()
                ));
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            _ => {
                let a = PaxOperator {
                    name: op.as_str().trim().to_string(),
                };
                let r = rhs?;
                let exp = PaxExpression::Prefix(Box::new(PaxPrefix {
                    operator: a,
                    rhs: Box::new(r),
                }));
                Ok(exp)
            }
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            _ => {
                let a = PaxOperator {
                    name: op.as_str().trim().to_string(),
                };
                let l = lhs?;
                let exp = PaxExpression::Postfix(Box::new(PaxPostfix {
                    operator: a,
                    lhs: Box::new(l),
                }));
                Ok(exp)
            }
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            _ => {
                let a = PaxOperator {
                    name: op.as_str().trim().to_string(),
                };
                let l = lhs?;
                let r = rhs?;
                let exp = PaxExpression::Infix(Box::new(PaxInfix {
                    operator: a,
                    lhs: Box::new(l),
                    rhs: Box::new(r),
                }));
                Ok(exp)
            }
        })
        .parse(expr)
}

/// Compute a pax expression to a PaxValue
pub fn compute_paxel(expr: &str, idr: Rc<dyn IdentifierResolver>) -> Result<PaxValue, String> {
    let expr = parse_pax_expression(expr, idr.clone())?;
    expr.compute(idr)
}
