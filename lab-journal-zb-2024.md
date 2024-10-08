
### 2024-08 - Initializing pax-designer

[x] Merge / rebase expression interpreting

# back up branch
git checkout -b zb/designer-bak-00

    migrating paxcorp/pax-designer to pax/pax-designer, including range of commits 561666d08256e52f967a6d59a3948175181cbf71 to 7767d5d4396ac8023cc456d6dda63151e076cd04 (inclusive)
    migrating paxcorp/pax-design-server to pax/pax-design-server, including range of commits 02ae043657ec1bf6053af0ad2de98041f20f6bee to 137ea86a1e27bbc84530c670f53291d784c80047 (inclusive)
    
    cd /Users/zack/code/paxcorp
    git format-patch 561666d08256e52f967a6d59a3948175181cbf71~1..HEAD --stdout -- pax-designer > pax-designer.patch
    git format-patch 02ae043657ec1bf6053af0ad2de98041f20f6bee~1..HEAD --stdout -- pax-design-server > pax-design-server.patch
    
    Apply patches: [see Solution]
    
    [error, patch does not apply]
    next to try: extract my local commits to patch; apply in the other direction on latest master
    
    [Solution]
    git apply --3way ../pax-designer.patch
    git apply --3way ../pax-design-server.patch
    
    Update init logic, two manifests, two designtimes on other side of serialization
    
    git checkout main  # or the branch where pax-designtime exists
    git subtree split -P pax-designtime -b temp-pax-designtime

if we’re in main and designtime
(AND this is not PaxDesigner itself — note that this code will be included in #[main] logic,
which we’ll have at least two of: userland and designer)
then parse PaxDesigner to manifest alongside parsing the userland component tree
deserialize, then
initialize a definition_to_instance_traverser from each manifest (each of which surfaces a get_main_component)
the <PaxFrame> component just traverses this singular boundary (register-ed) for now; can make extensible later with different cartridges
the root component for the engine should be PaxDesigner; the inner component is the userland component


**Dev harness:**

*TL;DR examples should just work the same as `designer-project`*

Would be nice to have a dev harness mechanism for pax-designer through this flow.  I.e. similar to
the designer-project flow we have today.  We seem to lose this because we're retiring designer-project

In fact, given the current setup with relative paths inside the monorepo, we might get this for
free with all existing examples.

They use a relative path for pax-engine, which uses a relative path for pax-designer,
which will thus recompile and update as we libdev.

sed -e 's|^--- a/pax-design-server/|--- src/design_server/|' -e 's|^+++ b/pax-design-server/|+++ src/design_server/|' ../../pax-design-server.patch | git apply --3way

bookmark:

commit abe247a528e87a569f1cd5dd6333c9eb61ba8339
Author: Zack Brown <zack@pax.dev>
Date:   Mon Aug 12 15:45:14 2024 +0700

    draft of macro init logic

[x] Init logic
[x] Macro init logic (compiletime)
[x] Issue: can't export pax_engine::pax_designer (circ. ref., because pax-designer depends on pax-engine)
    - could import pax-designer directly from userland, but adds to boilerplate
    seems most viable so far — maybe even correct to require `pax-designer` as a dev dependency when you intend to use it.
    we can handle generating this boilerplate for new projects, and carefully document it.  
    ^ trying the above
    now hitting an issue where the `pax_designer` crate (as generated by macro) doesn't exist within the `pax_designer` crate
    one possibility is to detect at macro-time whether we're in a whitelisted crate
    - could pre-parse pax-designer to manifest, including the manifest string as a compiler artifact (string constant)
    then at init-time, instead of parsing pax-designer to manifest, just load the string
    [Problem] this works for parsing — but it seems we will still need pax_designer in the module tree at runtime (probably build-time, too)
    so that e.g. event handlers & properties are present.
    This further suggests figuring out a way to reexport pax_designer through pax_engine, if possible...
[-] could not import pax_engine inside pax_designer; instead directly reach into all reexported packages
would enforce that pax_engine is a nice thin wrapper; nothing contained directly
    ^ going to give this a shot...
    [Nope], this tangles with the assumption that `pax_engine` exists at macro-time, which would break #[pax] inside
    pax-designer.  (could parameterize this symbol, but more complexity, esp. in generated cartridge)
[-] could we bundle designer under pax-std?  The source of the circ ref is pax_designer -> pax_engine -> pax_designer, so [the issue remains]
[-] what about a new crate: pax-engine-without-designer.  pax-designer depends on this.  it is the current pax-engine crate.
    then pax-engine pub uses the above crate, as well as pax-designer
    finally, userland depends on pax-engine, and gets both crates

        Hypothesis: workspace is failing to build, and this is causing spurious errors
            - get workspace building,
            - come back and try `cargo run --bin=parser --features=parser,designtime` on fireworks

        rn pax-chassis-web has designtime OFF
        and pax-runtime has designtime ON
        start in fireworks, track down flags & deps; see who's breaking the chain

        [Solution] — default = ["designtime"] was added to pax-engine/Cargo.toml; was causing break
        [Solution cont.] — remove pax-designer from root workspace; enables building workspace without `--features designtime`
    
[x] Issue: we are including cartridge.partial.rs across every #[main], which e.g. causes build of pax-designer to fail
    when running Fireworks.

    Drafted solution:
        [x] detect whether we are in the root crate of this build.
        [x] might be able to store a static mutable Option<root_crate_pkg_name>, a write-once-read-many (WORM) signal to the rest of the build.
        [x] in the stpl template, check this signal and only include the partial if we are in the root crate. 
            [-] This might be fragile if somehow different versions of pax-macro are included in a build (is that possible or does cargo prevent it?) Answer: cargo prevents it. (answer ft. copilot)

    Note: as of current state, building without designtime requires omitting pax-designer as dependency. 
            might want to rewrap pax-engine-without-designer and pax-engine in another crate after all, to ease ergo here? (consolidating flags:  --feature=designer) 
            ^ this can happen in another pass after big merge

    Success for this stretch will look like: can build fireworks and hopefully even run it; expected runtime crash

    Looks like there's an issue with `IS_DESIGNTIME_BUILD` definition — checking cfg within pax-compiler likely won't be useful.
    Probably should pass a runtime flag via CLI instead.
    ^ Resolved this but still hitting
    ```
    6 | #[pax]
    | ^^^^^^ use of undeclared crate or module `pax_designer`
    ```
    This is despite the fact that we're including pax_designer in the root crate.  Could it be because (a) the crate is failing to build,
    or (b) there's some corruption of features, or (c) because pax_designer isn't present in the workspace, even though it's available via the relative path specified in the root project's Cargo.toml?

[x] Compiler-side: deserialization updates (Vec<PaxManifest>)

[x] achieve build and run (cargo config, parser, etc.)
    `error[E0433]: failed to resolve: use of undeclared crate or module `pax_designer`
    
    perhaps pax_designer is not being exported or `pub use`? (it seems to be doing so correctly)

    `cd pax-designer && cargo build --features=parser` fails:
    ```
    warning: `pax-designer` (lib) generated 1 warning
    error[E0601]: `main` function not found in crate `parser`
    --> src/lib.rs:163:2
    |
    163 | }
    |  ^ consider adding a `main` function to `src/lib.rs`
    ```

    Clue: running `cargo run --features=parser` fails on pax-designer.  Digging through output, found this error:
    Location: [35m/Users/zack/.cargo/registry/src/index.crates.io-6f17d22bba15001f/pax-compiler-0.22.0/src/lib.rs
    
    Which suggests there's a place we're pulling a local crate from crates.io instead of via fs path!
    There are no apparent violations in Cargo.tomls, from what I can find

    What apparently finally resolved this was the combo of: 1. fixing a double newline in the .stpl, which was breaking a feature flag cfg macro, and 2. running cargo update

    BUT we still have the `pax_designer` import issue `failed to resolve: use of undeclared crate or module `pax_designer``

    signing off for the day aug 14 2024 with a victory: achieved build of fireworks + designtime; it also runs (rendering only userland component, not designer); now must manage runtime init with the two definitiontoinstancetraversers
    (see 7c169db2a09d69d8607aeed933149a3d791d562f)
            
[x] Engine init logic (runtime)
    [x] for designtime builds, register both definitiontoinstancetraversers with engine on init
        [-] stub out pax-designer designtime for native builds for now (just render userland project, even if designtime is on)
    [x] for designtime builds, render the root component via the designer; register the userland component for paxiframe
    [x] render userland component via paxiframe

[x] Resolve scoping of addressable components via designtimemanager 
    - resolved by managing merged_manifest separately from userland_manifest;
    we use the merged_manifest for codegen and userland_manifest for the DTIT and designtimemanager

[x] InlineFrame primitive
    [x] don't worry about string lookup for now - just pull from register
    [x] add to designer in both places currently exists (see stubbed TODOs)
    [x] handle transition between DTITs as elegantly as possible (set children statefully a la frame?)

[x] run design_server instead of static server
    [x] refactor and consolidate divergent building:: vs design_server:: logic
        [x] assess what else is exposed&expected (e.g. websockets, priv. agent)
        [x] figure out  `std::env::set_var("PAX_WORKSPACE_ROOT", "../pax");`
        [x] Expose previous bin logic through method (env, mostly)
            [x] must also solve async runtime (since not using tokio::main)
        [-] bolt onto existing static serve logic (e.g. with port-seeking)
            [-] refactor use of static server from `building` => `design_server`
    [ ] [NOTE: still unresolved] figure out port coordination + autoport (or fall back to fixed)
        [-] client-side, should be able to query port of current URL / file?  
            window.location.port, 
            or maybe even just a relative path
        [-] there's a tangle with how the designtimemanager is init'd —
            are we crossing a websys boundary and can we query client port at that time?
        [x] punted by hard-coding to 8080; will need more effort to support dynamic ports

[x] Root out ROOT_PROJECT_ID — must make dynamic; 
    - exposed get_userland_root_expanded_node

[x] solve `root.global_id().unwrap()` — sometimes global_id is None; why?
    - this blocks e.g. instantiating a Rect, probably also selection

    - observation: global_id() is None because we fail to build_template_node
    - likely: related to edge case of root_expanded_node not having a containing component ID

    uniquenodeidentifier is useful for everything EXCEPT the root
    everything except the root is a templatenode for some other component
    
    the issue: we rely on that uniquenodeidentifier
    Now that the userland_root is the root of the manifest,
    we can no longer use
    The only thing we CAN know is the typeid of the main component

    Possibly make uniquenodeidentifier an enum: {Root(probably need root component id here), Node(ContainingComponent, TemplateNodeId)}
    Alternatively, refactor the use of global_id in designer to have better awareness of root, instead of

[x] As a stopgap: in the manifest, put the extra component in there, which in its template just has the userland component
    when? could probably do at macro time, same place we add BlankComponent
    "root wrapper"
    in that component, put the userland root in there as a single template node
    in get_main_component, where we create the uniquetemplatenodeidentifier, 
    make it a combination of that special wrapper component's typeid
    and tnid(0) (the root)

[x] solve assets (can we merge all? do we need to coordinate across `#[main]`s somehow?)
    - wj: store fs paths in manifest?
        - for any #[main] component, can "stamp" asset fs path into manifest, place on FS to check for assets at build time
    - had to do some macro wrangling / surfing, but got there

[x] one more merge from dev; prepare PR
    [x] Previously patched up to `7767d5d4396ac8023cc456d6dda63151e076cd04`

        git format-patch 7767d5d4396ac8023cc456d6dda63151e076cd04..HEAD --stdout -- pax-designer > pax-designer.patch
        git format-patch 7767d5d4396ac8023cc456d6dda63151e076cd04..HEAD --stdout -- pax-design-server > pax-design-server.patch
        cd pax
        git apply --3way ../pax-designer.patch
        git apply --3way ../pax-design-server.patch
        
        [Solution]
        Manually edited paths in patch file to apply changes to updated locations in new repo
        used `--reject` to allow the merge to continue, dumping .rej files that I could patch manually

[x] Module cleanup pass
    [x] pax_kit naming
            engine
                api
                manifest
                etc.
            designer
            pub use pax_engine::* ???

        Possible names for bundled userland crate:
            pax_sdk
            pax_api
            pax_kit
            pax_main
            pax_all
            pax_tools
        
        trying out pax_kit: 
        pax_kit is designer plus engine
        engine is what it is (pub use * it for api forwarding) 
    
    [x] use pax-kit in userland and in examples (start with fireworks)
        pax-kit should not be a dependency of any crate in the monorepo, expect for pax-designer (which is a userland crate)

        the pax-std puzzle: should we bundle pax_std inside pax_kit?  (probably?)
        this forms a wrinkle where we link primitive instances vs. properties structs

        e.g.
        ```
        #[pax]
        #[primitive("pax_std::core::inline_frame::InlineFrameInstance")]
        ```

        [is this linking necessary anymore????? can we consolidate these structs? doing this should solve the wrinkle]
        [yet another, possibly far simpler option is to hard-code the `pax_kit::` prefix on primitives] 


    [x] codegen: deal with the hard-coded assumed root symbol (pax_engine)

        need to parameterize this symbol in order to support designer libdev (or can this be reflected automatically from pax-macro? detect module path of pax-macro itself, use that prefix for codegen)

        must parameterize separately, to make its way to Tera (compiletime)

        alternatively... maybe we can get away with swapping to pax_kit for now?  This seemingly only doesn't solve pax-designer running standalone, but that may be a non-goal

        start here, in pax-macro/src/lib.rs: `let path = module_path!();`

        then find/replace/parameterize through templates (check both stpl and tera)

        another pax-std puzzle:  pax-std depends on pax-engine, (which is reasonable, as opposed to depending on pax-kit) — but this messes with codegen,
        when we assume pax_kit is in scope e.g. by codegenning `pax_kit::pax_engine::`...
        in other words, pax-std needs to codegen a different prefix than userland (pax-designer probably needs to do the same)
        one way to do this is with a macro param, something like:
        ```
        #[pax]
        #[engine_import_path("pax_engine::")]
        ```
        problematically, this probably needs to be sorted / split

[x] achieve non-designtime builds

    [x] shuttle feature flags:  features=["designer"] => designtime feat + dynamic deps (pax-designer)
        update features & optional deps to include / exclude designer

    [x] update examples cargo.tomls

[x] e2e journey: fix pax-cli `build` x {non-designer}

    to test, run loop of publishing to crates.io and testing

    publishing pax-designer and pax-kit for the first time:

    when publishing pax-designer:

    Packaged 60 files, 36.1MiB (35.6MiB compressed)
    Uploading pax-designer v0.24.0 (/Users/zack/code/paxcorp/pax/pax-designer)
    error: failed to publish to registry at https://crates.io

    Caused by:
    the remote server responded with an error (status 413 Payload Too Large): max upload size is: 10485760

    Seems we need to trim 36.1MiB down to 10MiB or so (or perhaps reach out to crates.io support for an ad-hoc limit raise)

    Turns out these guys were collectively ~33MiB:

    delete mode 100644 pax-designer/assets/images/map_paris.png
    delete mode 100644 pax-designer/assets/images/map_sf.png
    delete mode 100644 pax-designer/assets/images/remix-comp.png

    [x] get a stable build incl. necessary assets  
    [x] update get started docs with CLI commands

[ ] NUX ++
    [ ] stream diffs (colored) in design server output
    [ ] design project with text on stage as welcome tour, show the ropes
        with text & images, all editable, ideally in layouts
        [ ] describe ultrasync;
        [ ] describe how to play / pause / resize
    [ ] make pax-cli new also run
        [ ] print message with link to project FS root
    [ ] see code: copy path / open terminal / open editor through privileged agent
    [ ] git init template project
        [ ] figure out keeping .git-named directory as asset within git (maybe rename like Cargo.toml)
        [x] .gitignore

[ ] UI cleanup pass

[ ] testing
    [x] designtime / nondesigntime builds
    [x] libdev vs. in-the-wild (CLI flow)
    [ ] linux
    [ ] windows

[-] build times vs. runtime perf
    balance flags; wrap inside pax_kit, manually test


Testing build time of spacegame using:
`cargo clean && time ./pax run`


Run 1: debug profile (debug = false) on pax-kit

./pax run  183.49s user 21.86s system 247% cpu 1:22.93 total

Run 2: package."*" debug profile (debug = false) on spacegame

./pax run  73.12s user 12.47s system 130% cpu 1:05.45 total

Run 3: no debug profiles again:

./pax run  163.10s user 18.70s system 238% cpu 1:16.15 total

Run 4: no debug profiles; remove cfg! check for --release flag in pax-compiler:

./pax run  140.68s user 16.38s system 251% cpu 1:02.57 total
        
Run 5: don't pass --dev in for wasm-pack

./pax run  338.04s user 19.93s system 299% cpu 1:59.66 total

Run 6: no changes

./pax run  340.34s user 20.21s system 309% cpu 1:56.60 total

Run 7: set profile.dev => debug = false in pax-lang only

./pax run  331.71s user 19.82s system 297% cpu 1:58.32 total

Run 8: revert changes to pax-compiler/src/building/web.rs

./pax run  209.13s user 23.50s system 272% cpu 1:25.40 total
./pax run  180.52s user 21.31s system 242% cpu 1:23.23 total














