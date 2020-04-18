[33mcommit b937045b5808a0da473834035a595eea629c127c[m[33m ([m[1;36mHEAD -> [m[1;32mmeuko-reserve-tests[m[33m)[m
Author: Hamza Tokuchi <h@mza.jp>
Date:   Sat Apr 18 20:35:12 2020 +0200

    Squashed commits -> Added base test cases for reservable balance

[33mcommit 5efcb0f94c3c513dfba290937ccc3751853f7759[m[33m ([m[1;31mupstream/master[m[33m, [m[1;31mupstream/HEAD[m[33m, [m[1;31morigin/master[m[33m, [m[1;32mmaster[m[33m)[m
Author: Hamza Tokuchi <h@mza.jp>
Date:   Thu Apr 9 15:53:31 2020 +0200

    Lock -> unlock event | reservable-currency pallet (#208)
    
    Changed the event call in the unlock function to the correct one.

[33mcommit 92823224ce347d2caa265e83c10591521440e54f[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Thu Apr 9 08:15:37 2020 -0400

    Add maps recipe to ToC (#207)

[33mcommit 28ac9694a4f6fb7377bd569b75214f21fde026ab[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Mon Apr 6 12:17:02 2020 -0400

    Remove files left over from debugging basic pow (#206)

[33mcommit 3e8f1f56382cd78a37556a1fdc4618eb2f53fdae[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Mon Apr 6 12:07:17 2020 -0400

    Update recipes to alpha.5 (#203)
    
    * Mostly working code, sketch of writeup
    
    * update Cargo.toml s to ba5c85a
    
    * Fix runtimes
    
    * Fix weights
    
    * Fix kitchen node
    
    * Fix rpc node
    
    * Update deps to 0bd9ffa
    
    * update compiler
    
    * Add dedicated pow runtime. basic-pow now builds and runs.
    
    * Clean code comments
    
    * strip down pow-runtime
    
    * Clean up genesis
    
    * Raise the difficulty
    
    * More write-up
    
    * simple -> basic
    
    * More writeup
    
    * Finish drafting writeup.
    
    * link checker adjustment
    
    * Update nodes/basic-pow/src/pow.rs
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update runtimes/pow-runtime/src/lib.rs
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update book.toml
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update nodes/basic-pow/src/chain_spec.rs
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Clean some commented code
    
    * Bring simple-map up to snuff (grr this is mixed with a bigger PR, oh well)
    
    * Update all cargo files.
    
    * Update basic-token (except tests)
    
    I'm not fixing the tests unless I learn that `MigrateAccount` type is
    sticking around long term.
    
    * Update struct storage
    
    * update execution-schedule
    
    * Ditch Linked Maps
    
    * Everything compiles (excet tests)
    
    * Simplify key and address handling in chain-spec
    
    * Update runtime toward apps compatability
    
    * Include this javascript diagnostic stuff temporarily.
    
    * Fix doc typo (Just so CI runs again)
    
    * Make pow-runtime code look as close as possible to api-runtime. (flailing)
    
    * Update to 34c5cb3. Everything compiles and tests pass.
    
    * Update to 1d8fa43 (all compiles)
    
    * restore Cargo.lock
    
    * Fix pow-runtime
    
    * de-alias super-runtime (finally)
    
    * Update deps to crates.io (alpha.5)
    
    * fix in-code build scripts alias
    
    * clean basic-pow cargo.toml
    
    * Clean Kitchen node cargo.toml
    
    * Include missing struct storage type
    
    * Clean rpc-node
    
    * Update version in Readme
    
    * clean default-instance
    
    * clean last-caller
    
    * Substrate is on crates.io Hooray!
    
    * Fix tests on new recipes
    
    * ocw runtime conforming to alpha.5 style (#205)
    
    * Add ocw-runtime in the top level virtual workspace
    
    * [wip] ocw-runtime
    
    * updating code to conform to current style
    
    * Update text/3-entrees/storage-api/structs.md
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    * Update text/3-entrees/storage-api/storage-maps.md
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    * Update text/3-entrees/storage-api/storage-maps.md
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    * Update text/3-entrees/storage-api/storage-maps.md
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    * Update text/3-entrees/storage-api/storage-maps.md
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    * Update text/3-entrees/storage-api/storage-maps.md
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    Co-authored-by: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    Co-authored-by: Jimmy Chu <jimmy@parity.io>

[33mcommit 7318e64c9e656c78281678a2a456599bdfbeab3b[m
Author: Jimmy Chu <jimmychu0807@gmail.com>
Date:   Tue Mar 31 13:08:03 2020 +0800

    Remove linked-map residual

[33mcommit fd1eeb85312450f8881637de1245d1fa6771474c[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Mon Mar 30 14:06:55 2020 -0400

    Comment improvements (#200)
    
    * Comments to single value
    
    * Comments in generic-event

[33mcommit 1a8190df2e22aa080d740c6aa0b3148b5cf17b64[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Mon Mar 30 09:25:45 2020 -0400

    Docs and style cleanups for Constant Config

[33mcommit 5e1da6d50533c9ff484dc73edc194cada97c13da[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Mon Mar 30 09:11:32 2020 -0400

    Remove broken link from Summary

[33mcommit 442c8ba7a6097e1da5e56a43baf633ab24356cf9[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Mon Mar 30 07:41:05 2020 -0400

    Fixed Point recipe (#196)
    
    * Sketching an idea
    
    * clean up source of copy-pasta
    
    * Dispatchable functions fleshed out. Pallet compiles. No exponential yet.
    
    * scaffold tests
    
    * Discrete interest tests
    
    * Types
    
    * Try and fail to use substrate-fixed
    
    * chekpoint
    
    * OMG it compiles!
    
    * Sketch out accumulator example
    
    * Test manual accumulator
    
    * checkpoint
    
    * Permill accumulator implementation
    
    * Fixed impl almost complete. One failing test.
    
    * fixed accumulator implementation complete.
    
    * Install pallets in super-runtime
    
    * Fix confusing compiler warning.
    
    * runtime types
    
    * Sketch write-up
    
    * Write up accumulators
    
    * remove FPBalance type
    
    * Write up discrete interest
    
    * Write up continuous interest
    
    * Fix runtime-api link
    
    * Strip unrelated broken links to get CI to pass.
    
    This is a stopgap measure to remove broken links. This article has been
    properly removed in the follow-substrate-master branch, but not merged
    here yet :-/
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: Peter Goodspeed-Niklaus <coriolinus@users.noreply.github.com>
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: Peter Goodspeed-Niklaus <coriolinus@users.noreply.github.com>
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: Peter Goodspeed-Niklaus <coriolinus@users.noreply.github.com>
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: Peter Goodspeed-Niklaus <coriolinus@users.noreply.github.com>
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: Peter Goodspeed-Niklaus <coriolinus@users.noreply.github.com>
    
    * clean up ensure_signed calls
    
    * Spoiler: substrate-fixed has more to offer.
    
    * add some detail about non determinism
    
    * Revert "clean up ensure_signed calls"
    
    This reverts commit a88f091e35149efd9bd088baff32dcde79e8d167.
    
    * Properly clean up ensure_signed calls.
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/fixed-point.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * improve substrate-fixed dependency
    
    * spelling: principle -> principal
    
    Co-authored-by: Peter Goodspeed-Niklaus <coriolinus@users.noreply.github.com>
    Co-authored-by: joe petrowski <25483142+joepetrowski@users.noreply.github.com>

[33mcommit ec5b2fc1362638c6c9063ec73e0b7956b814fce6[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Mon Mar 30 19:23:48 2020 +0800

    Adding off-chain worker runtime (#198)
    
    * Remove offchain_demo from super-runtime.
    
    * Added ocw-runtime and fixing compiler warning messages.
    
    * minor phrasing
    
    * cleaner warning suppression
    
    * build instructions
    
    * fix link
    
    Co-authored-by: Joshy Orndorff <admin@joshyorndorff.com>

[33mcommit fd43d31e66dd24ec70b8b4f50e68efdcf17766f3[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Sat Mar 28 04:22:33 2020 -0400

    Ditch linked Maps (#199)
    
    * cherry pick 1f1788e and resolve conflicts
    
    * Fix table of contents

[33mcommit 2442ed9df5d59fdaba15faa49e55c7820ce94f4c[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Wed Mar 25 11:46:12 2020 -0400

    Fix link checker again. (#197)

[33mcommit 4c429188ca68d228f9147420419fd4bf3c27bf21[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Wed Mar 25 09:54:27 2020 -0400

    Little clean-ups to Basic Token (#194)
    
    * Fix link
    
    * typo
    
    * Emphasize the don't panic idea.
    
    This used to be emphasized more prominently but I've accidentally pruned
    it in the restructuring.

[33mcommit 12f8c549fa5a92d51d18bd7a722932118a179415[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Wed Mar 25 09:19:36 2020 -0400

    Simplify genesis configurations (#193)
    
    * Simplify super runtime
    
    * Simplify weight-fee runtime
    
    * Simplify api-runtime
    
    * Simplify PoW runtime
    
    * Update text
    
    * remove on last reference to a genesis crate.

[33mcommit 1320cc5e9d63c60d5ecfdb3765ce0579226cce2d[m
Author: kaichao <kaichaosuna@gmail.com>
Date:   Wed Mar 25 19:32:26 2020 +0800

    fix typo (#195)

[33mcommit 315c5f23859f60d73f30239011bcd90de1aec72c[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Mar 24 13:41:15 2020 -0400

    Minor cleanup to Hello Substrate

[33mcommit 018eb3f4bd881bb7a42584806124c6dcd9b8d49c[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Mar 24 13:39:07 2020 -0400

    Fix caps

[33mcommit 72cbb7b6fdfe3370b7918f0f77d1dc3059e28f64[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Mar 24 13:04:07 2020 -0400

    Fix link to book in Readme

[33mcommit d981928871995ddbf8d0e67a77f9ccddc6060442[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Mar 24 13:02:12 2020 -0400

    Fix Substrate version in Readme

[33mcommit c55f8d7d43c0e8f14767b0c9af3605c13173184a[m
Author: Alexander Popiak <alexander.popiak@gmail.com>
Date:   Tue Mar 24 13:21:47 2020 +0100

    add ringbuffer queue recipe (#180)
    
    * add ringbuffer queue to pallets
    
    * remove code and add tests
    
    * add ringbuffer to Cargo.lock
    
    * convert simple-map deps to new format (inline)
    
    * add sp-std to deps and use new format
    
    * add ringbuffer-queue to super-runtime
    
    * update UI types and add unique name for BufferIndex
    
    * Sketch writeup in text
    
    * start writing ringbuffer explainer text
    
    * make dispatchable functions public
    
    * add link to FIFO wikipedia page
    
    * radically simplify ringbuffer trait
    
    * explain WrappingOps trait
    
    * link to correct markdown file
    
    * describe transient struct
    
    * add rest of implementation text as well as usage
    
    * fake commit to trigger github actions
    
    * Revert "fake commit to trigger github actions"
    
    This reverts commit 641016acf6e15029d66b4291269c506f8e26d650.
    
    * adjust the link checker.
    
    * Update text/3-entrees/storage-api/ringbuffer.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/storage-api/ringbuffer.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/storage-api/ringbuffer.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/storage-api/ringbuffer.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * use quote to separate notes and asides
    
    Co-authored-by: Joshy Orndorff <admin@joshyorndorff.com>
    Co-authored-by: joe petrowski <25483142+joepetrowski@users.noreply.github.com>

[33mcommit a25ba493312b6ce4dcd3465883368e2ba9767a60[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Tue Mar 24 20:12:09 2020 +0800

    Explanation on PoW finality block is 0 (#189)
    
    * Explanation on PoW finality block is 0
    
    * Update text/3-entrees/basic-pow.md
    
    * Update text/3-entrees/basic-pow.md

[33mcommit 06ae36342bf1a733c07fa6fabcdca2e389c6b219[m
Author: boneyard93501 <4523011+boneyard93501@users.noreply.github.com>
Date:   Mon Mar 23 15:11:52 2020 -0500

    update Docs glossary links (#187)

[33mcommit 23d7be70cfb8dd90aa627c6caa70d6e2038368ef[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Mon Mar 23 15:11:20 2020 -0400

    Recipe for Proof of Work node (#173)
    
    * Mostly working code, sketch of writeup
    
    * Add dedicated pow runtime. basic-pow now builds and runs.
    
    * Clean code comments
    
    * strip down pow-runtime
    
    * Clean up genesis
    
    * Raise the difficulty
    
    * More write-up
    
    * simple -> basic
    
    * More writeup
    
    * Finish drafting writeup.
    
    * link checker adjustment
    
    * Update nodes/basic-pow/src/pow.rs
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update runtimes/pow-runtime/src/lib.rs
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update book.toml
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update nodes/basic-pow/src/chain_spec.rs
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Update text/3-entrees/basic-pow.md
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    
    * Simplify key and address handling in chain-spec
    
    * Update runtime toward apps compatability
    
    * Include this javascript diagnostic stuff temporarily.
    
    * Fix doc typo (Just so CI runs again)
    
    * Make pow-runtime code look as close as possible to api-runtime. (flailing)
    
    * minor edit and spacing (#184)
    
    Co-authored-by: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    Co-authored-by: Jimmy Chu <jimmy@parity.io>

[33mcommit b7407f53f50a9df7bbf83d4c10b75e85221ecf02[m
Author: Hamza Tokuchi <h@mza.jp>
Date:   Mon Mar 23 19:47:35 2020 +0100

    Deepen path to runtimes, rebuild book please -> mention::https://github.com/Meuko/recipes/blob/master/text/1-prepare-kitchen/3-kitchen-organization.md#inside-the-kitchen-node (#186)

[33mcommit 5a1c8feef0bbf7edcf88f7022d425eaae9ce29c5[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Mon Mar 23 07:52:09 2020 -0400

    Bring Randomness Recipe up to snuff (#182)
    
    * Fixes strait-forward compile errors, but doesn't update randomness api
    
    * Pallet that compiles and probably partially works
    
    * Code working
    
    * typo
    
    * draft writeup
    
    * add citations and rabbit hole section
    
    * Address some feedback from review.
    
    * Review comments to code.
    
    * Some basic tests.
    
    * update links in the sidebar and content at the bottom
    
    * updated book.toml
    
    Co-authored-by: Jimmy Chu <jimmychu0807@gmail.com>

[33mcommit da3c108ee6a8107cc9fab65d10d87b286aa50f39[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Fri Mar 20 15:39:39 2020 -0400

    Fix weights in all runtimes (#183)
    
    * Fix weights
    
    * And a typo
    
    (Just so CI runs again)

[33mcommit c8caa7a0a0840875503bdfcf3ee0f3f51501aeab[m
Author: Jimmy Chu <jimmychu0807@gmail.com>
Date:   Wed Mar 18 20:17:44 2020 +0800

    typo fix

[33mcommit 2402cbf1d518686b27ce8846d5840688ba100dc7[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Mar 17 04:59:06 2020 -0400

    Offchain followup (#177)
    
    * de-alias crates
    
    * Add doc comments
    
    * Standardize link to pallet in kitchen
    
    * Link to sc_service
    
    * "configuration trait"
    
    * Remove duplicate link
    
    * fix tests
    
    * Fix test warnings
    
    * Updating the code comment
    
    Co-authored-by: Jimmy Chu <jimmychu0807@gmail.com>

[33mcommit 8e9bcc7ad9e2b49e5aeb75a1f9f09f2a46d990d7[m
Author: Jimmy Chu <jimmychu0807@gmail.com>
Date:   Mon Mar 16 16:40:13 2020 +0800

    Fix off-chain worker test links

[33mcommit ccfd7b948897d268486fc9ae3c2c83200d39b079[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Mon Mar 16 16:28:03 2020 +0800

    off-chain workers recipe (#162)
    
    Co-Authored-By: joe petrowski <25483142+joepetrowski@users.noreply.github.com>
    Co-authored-by: Joshy Orndorff <admin@joshyorndorff.com>
    Co-authored-by: joepetrowski <joe@parity.io>
    Co-authored-by: joe petrowski <25483142+joepetrowski@users.noreply.github.com>

[33mcommit dfc931c4dcad190790728c87c27a1734084c9de7[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Sun Mar 15 21:36:20 2020 +0800

    Fixing README.md links in doc content (#176)
    
    * Fixing README.md links in doc content
    
    * Changed README.md all to index.md

[33mcommit b413f78ff21440125f61730e457bf33c0afb6435[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Sun Mar 15 08:03:27 2020 -0400

    Kitchen node uses build-script-utils (#172)

[33mcommit fc4c625addf43cb5088a73f6f0559db24fc2bd95[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Tue Mar 3 08:28:05 2020 -0500

    Fix broken link in child trie

[33mcommit de10dbfc32bc74cb353de1b223c045e56b7d9efc[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Mar 3 08:22:25 2020 -0500

    Update to Substrate v2.0.0-alpha.3 (#165)
    
    * adding machine works
    
    * Basic Token up to snuff
    
    * Charity works
    
    * check-member-ship works
    
    * Child trie works
    
    * Constant config works. tests separated.
    
    * currency imbalances still works
    
    * default instance still works
    
    * Double map works
    
    * execution schedule works. Tests separated.
    
    * Update substrate reference to tag.
    
    * gen-random still vestigial
    
    * generic-event works
    
    * hello-substrate works
    
    * last caller still works
    
    * Linked map works
    
    * lockable-currency works
    
    * reservable-currency works
    
    * simple event works
    
    * simple map works; tests separated; de-aliased
    
    * single value works
    
    * simple crowdfund still vestigial
    
    * storage cache works
    
    * struct-storage works; tests separated, de-aliased.
    
    * sum-storage works; tests separated
    
    * vec-set works
    
    * Weights works; spaces to tabs
    
    * Guess it's time to commit the Cargo.lock
    
    * fix simple map features
    
    * super-runtime compiles
    
    * super genesis compiles
    
    * super-runtime: spaces -> tabs
    
    * weight-fee runtime (and genesis) compiles
    
    * automated text update
    
    * api runtime (and genesis) compile
    
    * Attempt to update kitchen node
    
    * Making kitchen-node compile & sum-storage test pass
    
    * cargo test run with rpc-node commented out
    
    * Update to use a more latest version of Rust
    
    * Update RPC Node
    
    * Yoink Substrate's .editorconfig
    
    * Bump to alpha.3
    
    * Fix Apps compatability
    
    Co-authored-by: Jimmy Chu <jimmychu0807@gmail.com>

[33mcommit d400a53e09829963b648788b25bc6557396ca991[m
Author: Terry Teh <43627527+bertstachios@users.noreply.github.com>
Date:   Sat Feb 29 00:57:48 2020 +1100

    Hello substrate documentation change to include a flag for the print function & the differences between running the debug::info! macro. (#161)

[33mcommit cb76aa86e714a2655fb6b3b1d83178bae6f7796c[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Fri Feb 21 12:55:28 2020 +0800

    3.1.4 code update & add a section on throwing generic error (#158)
    
    * 3.1.4 update
    
    * Adding a section throwing generic errors
    
    * minor update
    
    * Update on testing storage items
    
    * Enable option to check external web links and fix broken links
    
    * Excludes link check to a paper
    
    * Updated per feedback
    
    * Fix errors

[33mcommit 0504599abc9f2ed9b4841f4705c5af81b1e2a391[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Tue Feb 18 12:46:19 2020 +0800

    minor format update

[33mcommit b5319eee6d78f6d2bec7fb0602ad7ba5c1fbc337[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Tue Feb 18 09:36:12 2020 +0800

    Jimmy/content update (#156)
    
    * typo fix
    
    * Wrote 1-hello-substrate
    
    * Wrote 2-storage-values
    
    * Wrote 3-errors
    
    * Wrote 4-events
    
    * Fix minor inconsistency in TOC (not really part of appetizers work)
    
    * Finish Appetizers content summary
    
    * Restructure entrees TOC
    
    * Remove accidental file
    
    * storage-cache update
    
    * Minor update
    
    * update on vec-set
    
    * Add a simple way to inspect variables
    
    * minor update
    
    * minor update
    
    * Update text/2-appetizers/1-hello-substrate.md
    
    Co-Authored-By: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
    
    * Update text/2-appetizers/1-hello-substrate.md
    
    Co-Authored-By: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
    
    Co-authored-by: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>

[33mcommit 0227754b39f00c3aebf054f51b8e50d9d95958f7[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Sun Feb 16 23:38:40 2020 -0700

    Followup / Cleanup to Appetizers PR (#154)
    
    * Clean up event imports in testing modules.
    
    * Typo fix
    
    * Insert architecture image
    
    * Tangential: rename other image

[33mcommit 518115d550b9393a23a2911f2956372caf9bbbdd[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Sun Feb 16 14:48:10 2020 -0700

    Adds link-checking to build (#155)
    
    * Configure linkchecker
    
    * Fix Links
    
    * Attempt CI
    
    * remove stray line
    
    * try fix syntax error
    
    * try again
    
    * better formattingformat and try bash
    
    * Fix build path
    
    * Less confusing name

[33mcommit 40b147757a0258470c088843e2a8acdb6955eb1b[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Sun Feb 16 00:34:13 2020 -0700

    Fill in Appetizers section (#152)
    
    * Wrote 1-hello-substrate
    
    * Wrote 2-storage-values
    
    * Wrote 3-errors
    
    * Wrote 4-events
    
    * Fix minor inconsistency in TOC (not really part of appetizers work)
    
    * Finish Appetizers content summary
    
    * Restructure entrees TOC
    
    * Remove accidental file
    
    * Update pallets/adding-machine/src/lib.rs
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    * assert_err -> assert_noop
    
    * Update text/2-appetizers/1-hello-substrate.md
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    * Update text/2-appetizers/2-storage-values.md
    
    Co-Authored-By: Jimmy Chu <jimmy@parity.io>
    
    Co-authored-by: Jimmy Chu <jimmychu0807@gmail.com>

[33mcommit cd0307ab5d83ae992394c52a06b86f5cee83c5b9[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Sun Feb 16 09:59:41 2020 +0800

    Testing Content Update (#150)
    
    * typo fix
    
    * Updated mock runtime setup
    
    * separate tests in `tests.rs`
    
    * Split up `test.rs` in generic-event pallet
    
    * Updated with better logical unit in the chapter
    
    * Updates regarding to feedback

[33mcommit bdf6b56f61d3c0cc8ac170d0fdd07859daafd522[m
Author: Jimmy Chu <jimmy@parity.io>
Date:   Thu Feb 13 01:02:58 2020 +0800

    typo fix (#147)

[33mcommit 8267a254c71df0cc81bee3cb222fe6ddc34a67fe[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Mon Feb 10 08:48:39 2020 -0500

    The Big Restructure (#143)
    
    * big directory restructure
    
    * subtitle
    
    * Prune more readmes that will inevitably get outdated
    
    * dedicated image directory
    
    * Add resources
    
    * Consolodate drafts
    
    * Prune token directory
    
    * Move desert (most of which is commented) to drafts
    
    * Move more-resources out of base directory
    
    * prune unused runtime.md file
    
    * move introduction to root level
    
    * Move more drafts to the drafts dir
    
    * Mega-commit. Primarily writes the preparing-your-kitchen section. Also moves the Learn Rust section.
    
    * Rebrand basics as appetizers, and write intro text.
    
    * Move execution schedule to prune tour directory
    
    * Revise Introduction
    
    * Correct link in summary
    
    * Remove useless pallet readme
    
    * Update links to removed kitchen/ directory throughout the text.
    
    * Minor typo fixes from Julie's review.
    
    * minor edit
    
    * Typo: bring -> begin
    
    Co-authored-by: Jimmy Chu <jimmychu0807@gmail.com>

[33mcommit 5172121ea4681ee061805d2d74d25b4af4ca2277[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Thu Feb 6 15:46:14 2020 -0500

    Correct indentation of Charity recipe in sidebar

[33mcommit d3256856f0bcb31a756f9018c68609b996a66cda[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Wed Feb 5 15:48:19 2020 -0500

    Rework simple treasury into Charity (#141)
    
    * work in progress
    
    * impl onUnbalanced
    
    * sketch simple spending routine
    
    * Pallet compiles
    
    * runtime compiles
    
    * de-alias pallet
    
    * Writeup for charity pallet.
    
    * tests
    
    * Try to fix build with specific rust
    
    * Actually fix build by removing strate feature gating.
    
    * Add storage
    
    * Revert "Try to fix build with specific rust"
    
    This reverts commit a2171e03d250ace42d2917b7182f725e0963b013.

[33mcommit a156ffdb2bcd3a44c2e0cf75bf743be6553031fd[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Feb 4 16:30:39 2020 -0500

    Remove directory of outdated remarks about utxo workshop (#140)

[33mcommit 1821a521cbd36ef73830345dabf7262f201d3f4e[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Mon Feb 3 16:51:33 2020 -0500

    Make recipes compatible with Apps (#137)
    
    * Basic type aggregation for super runtime
    
    * Everything compiles; just need type aggregation.
    
    * de-alias crates in kitchen node
    
    * Clean up merge that got f*cked by modules -> pallets rename
    
    * Instructions about types.

[33mcommit 6fdf64f3f0161e084be76c291c8efea3de35bb94[m
Author: Simon Schoeters <b2282e9d-46e8-4ec4-946a-b6ad84078b1c@astil.be>
Date:   Sat Feb 1 17:00:43 2020 +0100

    Update folder structure (#136)
    
    The folder structure seems to have changed and the path mentioned in the documentation no longer works.

[33mcommit c187378924da99617c6e191acd8e68bbc8dbf118[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Thu Jan 30 15:12:33 2020 -0500

    Demonstrate colelcting fees in seperate asset. (#135)
    
    * Allow fee collection in spending asset
    
    * Allow fee collection in fixed asset.
    
    * Split writeup into weights and fees.
    
    * duplicate content: sumary -> readme
    
    * Uncomment accidental comment line.

[33mcommit 524cecd42aa49f069bdf6f57679da03313c615a9[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Thu Jan 30 12:29:50 2020 -0500

    Recipes for custom Runtime APIs and RPCs (#129)
    
    * Bring in pallet
    
    * Start bringing in runtime. At least Cargo.toml refs resolve.
    
    * re-alias pallets
    
    * rename sum-storage-rpc crate
    
    * fix std feature (and thus compile)
    
    * Write up runtime API
    
    * Fix / Write pallet tests
    
    * Restructure for multiple nodes (and simplify session keys).
    
    * Little more text in rpc article
    
    * Code and write RPC
    
    * Clean unused imports
    
    * Demonstrate RPC parameters
    
    * Calling with optional parameters
    
    * Mention javascript and polkadot api
    
    * Revise runtime API writeup
    
    * Add writeups to sidebar.

[33mcommit b8aef161825326243418aeb0720ab9541e66fe98[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Fri Jan 17 10:35:23 2020 -0500

    Update to pre-2.0 and do some cleaning (#128)
    
    * Pallets and Runtimes compile
    
    * double-map: clean deps and tests
    
    * Checkpoint while fixing / cleaning tests
    
    * Fix simple treasury
    
    * tests pass, no warnings, results used correctly
    
    * check-membership: clean and de-alias deps
    
    * basic-token: de-alias dependencies
    
    * unfuck check-membership
    
    * Fully enable simple treasury pallet
    
    * constant-config: Remove unneeded balances dependency

[33mcommit e12df716561d1484b50da004466310f49359a317[m
Author: waylandc <wayland.chan@gmail.com>
Date:   Mon Jan 13 21:45:04 2020 +0800

    correct the directory containing target directory (#127)

[33mcommit 39dde2e547478f4c8e06206d553f6eb4810e3d74[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Fri Jan 10 12:42:41 2020 -0500

    Fix broken links (#126)
    
    * First few.
    
    * The rest.
    
    * Update src/declarative/ensure.md
    
    * Update src/storage/enumerated.md
    
    Co-authored-by: Jimmy Chu <jimmychu0807@gmail.com>

[33mcommit 636f6ca6f97b128463cbc186cf8a12ce8f96c131[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Fri Jan 3 15:12:14 2020 -0500

    Frame rename (#117)
    
    * rename modules directory
    
    * brute force search for srml
    
    * Remove remaining crates.parity.io links
    
    * scrub phrase "runtime module"
    
    * Update CI config for new pallets directory
    
    * second half of CI update 🤦‍
    
    * Contributing
    
    * module-constant-config -> constant-config
    
    * Checkpoint. Holy shit.
    
    * Light at the end??
    
    * fuxing with smpl-crowdfund. Gonna delay this until after rename PR.
    
    * Update kitchen/README.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update kitchen/pallets/last-caller/src/lib.rs
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update kitchen/pallets/smpl-treasury/README.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update src/base/README.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update kitchen/pallets/default-instance/src/lib.rs
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update CONTRIBUTING.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update kitchen/README.md
    
    * Update src/advanced/drafts/maps.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    Co-authored-by: Amar Singh <asinghchrony@protonmail.com>

[33mcommit 77fe7af04fb08b1ccb6bbc7b5e82bf7ab395e87f[m
Author: brenzi <brenzi@users.noreply.github.com>
Date:   Sun Dec 22 16:58:39 2019 +0100

    small fix (#122)

[33mcommit 2916042294d3c487f265f87083260e71f1d62bee[m
Author: Leo Arias <elopio@openzeppelin.com>
Date:   Thu Dec 19 21:09:29 2019 -0600

    Fix the links of the declarative recipes (#118)

[33mcommit d0c768422aefaec7aa5662691ded11bcfa9b3a1c[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Thu Dec 19 15:59:54 2019 -0500

    Speed up CI build/test with caching (#115)
    
    * take 1
    
    * syntax
    
    * fix copy pasta
    
    * just one cache
    
    * No hashing
    
    * Test hypothesis about config yml semantics, but mostly this is just to test whether the cache is resorted.
    
    * Try caching on target directory (seems like a less permission-problematic directory than ~/.cargo
    
    * Try some echo commands, see if target cache restores successfully/
    
    * try hardcoded kitchen path.
    
    * Include a Cargo.lock file for the node
    
    * Enable all three caches, fix target directory location, check entire project
    
    * Remove playing around

[33mcommit 5dc03e0d17932ec2a27d57f9f5e8620e8f34cdf8[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Dec 17 21:16:44 2019 -0500

    Devhub Backlink (#114)
    
    * Generate css, change favicon
    
    * Remove all theming except favicon
    
    * Restore theme line in book.toml
    
    * Install a mostly-working backlink to devhub.
    
    * remove the underline.

[33mcommit 7b6a70def56590f0dbe1011972b2f58e9f2bf0d2[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Dec 17 16:56:20 2019 -0500

    Update Substrate dependency to latest master (#107)
    
    * Rename dependencies
    
    * pin upstream substrate to `td-warnings`
    
    * Rename to reduce cognative overhead tx-pool-api -> sc-transaction-pool
    
    * ApplyExtrinsicResult #4143
    
    * Fix compilation of weight-fee-runtime with latest Substrate
    
    * sp-transaction-pool
    
    * Comment and format Weights pallet
    
    * Fix unused imports
    
    * Try and fail to make outer node compile
    
    * Try and fail to make super-runtime compile
    
    * Fixes to get it to compile with substrate master. (#113)
    
    * Newer mdbook
    
    * Live fixing done! :)
    
    * Tests fixed as well.
    
    * Pin to specific revision
    
    * Restore super-runtime as default runtime.
    
    * Mention PaysFee trait in writeup on weights.

[33mcommit 79f3d35c8d844af99cec965e96a717285d2491b5[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Dec 17 13:19:28 2019 -0500

    Theme to better match devhub (#84)
    
    * Generate css, change favicon
    
    * Remove all theming except favicon
    
    * Restore theme line in book.toml

[33mcommit efe6d1702222d1296b478d4bafa2d3137f6a3d95[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Dec 17 11:53:20 2019 -0500

    Port mdbook build to github actions (#111)
    
    * Idea from https://github.com/peaceiris/actions-mdbook
    
    * disable travis config
    
    * Also build on PRs
    
    * try with personal token
    
    * don't build on PRs
    
    * Remove travis config entirely

[33mcommit c3f00f86cb233397e535766bb5c779d703b16899[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Mon Dec 16 15:58:55 2019 -0500

    Newer mdbook

[33mcommit 4c5b927e250084adbf475ccc665659308a1a33d7[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Thu Dec 5 17:54:30 2019 -0500

    Add both runtimes and node to workspace. (#106)

[33mcommit 5af7ec5de73ca69558c6b9cf1391011ab1559bb8[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Thu Dec 5 09:05:24 2019 -0500

    Joshy cleanup execution schedule (#104)
    
    * Update test and observe it fails.
    
    * Fix signal bank calculation
    
    * Fix score calculation
    
    * Cleanup some simple warnings
    
    * cargo fmt
    
    * Move SimpleArithmetic import
    
    * Remove apparently unused line
    
    * Fix unused result warning
    
    * remove extraneous mut
    
    * move naive execution estimator, split tests, and add more commentary in src/

[33mcommit afde09957723d4a83b6b6468416233b2337fdce1[m
Merge: 4da4f33 d717f90
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Dec 4 16:17:21 2019 +0100

    Merge pull request #101 from substrate-developer-hub/improve-devx
    
    Improve Devex

[33mcommit d717f90cc79d96655f187f4e08ef68405fcae40b[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Wed Dec 4 09:31:31 2019 -0500

    Try pinning to the old v1 (still don't need submodules though)

[33mcommit b734470e7af525c4043b03e6e7da3d68fd698d99[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Wed Dec 4 09:28:15 2019 -0500

    Update checkout action to v2

[33mcommit 78db59518986b1bc2833d6f57cdd12b73841f3a4[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Dec 3 17:10:57 2019 +0100

    update testing/externalities with a link and note

[33mcommit a635869a719364e1965c662cecb9be9465dfd495[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Dec 3 14:46:06 2019 +0100

    change name token back to basic-token after noticing proc-macro token crate

[33mcommit 7b00784fa41b24edccdd137ae17c397d6abef652[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Dec 3 13:21:41 2019 +0100

    follow the preferences in this doc

[33mcommit cdc76cb3edc0f624c8a315f0e4731bbf9d03d326[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Dec 3 13:12:12 2019 +0100

    address review comments

[33mcommit 972a667ab83a8b10de209c4020c9b0ead36adc85[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Dec 3 12:49:22 2019 +0100

    remove unnecessary package param

[33mcommit e48b88808541a461a39e7288a9259fb072b94250[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Dec 3 02:26:30 2019 +0100

    update and add example of toml preferences

[33mcommit 18acca0718985bb3335f703ef63f8575065c949e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Dec 2 15:27:05 2019 +0100

    init

[33mcommit 4da4f33f8518b1606904dd03235fe4c44b576263[m
Merge: 09573e2 5f1c8b4
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Dec 2 12:27:19 2019 +0100

    Merge pull request #99 from substrate-developer-hub/new-button
    
    vanity metric and proper removal of social network

[33mcommit 5f1c8b443c0bad5c5dca4a1b426ad15437ed624c[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Dec 2 00:02:54 2019 +0100

    vanity metric and proper removal of social network

[33mcommit 09573e2d6eeaff887635364558eac9adb4ab7c8e[m
Merge: 9d524e0 fb15d2b
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun Dec 1 23:20:43 2019 +0100

    Merge pull request #87 from substrate-developer-hub/more-test-coverage
    
    Init Testing Chapter

[33mcommit fb15d2ba7605ca66bbf84f38efb16173912725bc[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Dec 1 17:30:53 2019 +0100

    fix event initialization instructions in instantiable modules recipe

[33mcommit 3668f746a7de8fefad4e178ce85882cf722bc2a9[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Dec 1 15:46:44 2019 +0100

    fix code blocks in instantiable-modules

[33mcommit 0c28b3ab498b499db2c33ab3cc36ef661e328406[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Nov 30 12:00:22 2019 +0100

    attempt to fix build, then ran cargo fmt

[33mcommit 8aa5da3fc11869c103098a8e3228d14e130df400[m
Merge: f3e9f79 9d524e0
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Nov 30 05:08:07 2019 +0100

    master.into()

[33mcommit f3e9f79eef6acb302caa0ff1ae661ca8de4b5e8a[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Nov 30 04:58:14 2019 +0100

    change rust annotations

[33mcommit a0bfaeb1188ea7fb6ce2d3580288f6a2bc4cdb0c[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Nov 30 04:44:25 2019 +0100

    fix all rust code block annotations to not allow user to compile and finish all other todos for this PR

[33mcommit eefd2db3771d03e05b09e11e59ca3766485f4db3[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Nov 29 17:22:34 2019 +0100

    delete social-network, add 2/3 of writing for new test chapter

[33mcommit 404ef47084418bcfcb6eecec03db3fe10ce8e64e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Nov 29 14:20:41 2019 +0100

    better test coverage for execution-schedule

[33mcommit 58fcc740cb5d52b8a6808617094eac791523c1e6[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Nov 29 03:52:05 2019 +0100

    execution-schedule test scaffolding and one test

[33mcommit f7d2393b0ad924a80ae40df376707d79e32e3ff9[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Nov 29 03:09:03 2019 +0100

    rewrite code for execution-schedule, next is tests

[33mcommit 3c2397d61ad130a0474e0c47ede5d35ca47b0c0c[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Nov 28 23:01:05 2019 +0100

    add struct-storage tests and cargo fmt

[33mcommit 94ec41510b10bf3265c30f996787ed09ee34cc05[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Nov 28 21:36:31 2019 +0100

    fixed up struct-storage, adding tests next

[33mcommit 4cd9c3850ef3aaeec6518fe2a75a946f426b4731[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Nov 28 16:44:35 2019 +0100

    trouble compiling execution-schedule

[33mcommit 9d524e0304a63932def9628048903c870db7f10f[m
Merge: 89de89d 204a32d
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Nov 28 12:00:54 2019 +0100

    Merge pull request #95 from 4meta5/master
    
    Github Actions CI

[33mcommit 204a32da62131e38724ca2d3f53de97e8c065de6[m
Merge: d9dbd30 bd5408d
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 20:09:46 2019 +0100

    Merge pull request #1 from 4meta5/add-check-to-delete-circleci
    
    Let's see how actions reacts to this PR

[33mcommit bd5408dcca257c57961ab9e575f531465856bce3[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Nov 27 20:09:19 2019 +0100

    deleted circleci

[33mcommit ed98a6f77a662f494caf060262ae1812f3c0b61b[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Nov 27 19:47:26 2019 +0100

    add check in test.yml

[33mcommit 7cd6841026dc8ac9b4cd4e766bf27b7d742947b0[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Nov 27 19:32:22 2019 +0100

    rename schedule_on_finalize to execution_schedule with intention of adding on_initialize as well && added changelog

[33mcommit d9dbd30a3ce989ffadb9ffbc4e08f9958d05538a[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 13:11:22 2019 +0100

    add status bar for github testing workflow

[33mcommit 7e44d01b720e5070b548ffe02f96948f5cf835bc[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 13:10:46 2019 +0100

    delete non-mdbook-build button from readme

[33mcommit 913ad60282f3000024012da4fa6b1c0f67ef3e40[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 12:27:56 2019 +0100

    update badges and delete superfluous recognition

[33mcommit 8e4f6dabdfd44d7270eca7a2ffba8f4b7a441988[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 12:24:33 2019 +0100

    don't check, just test
    
    can re-add check, but I think test compiles it anyway

[33mcommit 0b896b5ff6bf9518ade17bca01cdd3aa820c6aa6[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 12:01:41 2019 +0100

    Update test.yml

[33mcommit da0a0fd6bc1108ac4b7b21f1a2224fcebd30ccfa[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 11:57:10 2019 +0100

    Update test.yml

[33mcommit d3a53b07242fb8be70857a38861a60a9f4e7f6c5[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 11:20:07 2019 +0100

    Update test.yml

[33mcommit 8365cfd30347e914f26c963d7c4a215b9ddfb1f9[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Nov 27 11:05:10 2019 +0100

    Create test.yml

[33mcommit 15904e6e848436a94c02368d3a67c511c815a653[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 26 22:31:41 2019 +0100

    update wording for the changelog

[33mcommit 1bea23856e65a41679329d35dfa6067dfc0036b8[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 26 22:29:10 2019 +0100

    set intermediate objectives in the CHANGELOG.md, add more tests to smpl-treasury, and minimize readme length

[33mcommit 000a72f21811c51ea21620fb80f17674f3d75c37[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 24 12:56:09 2019 +0100

    add genesis config and test for it

[33mcommit 60c7f4b18ec0da2e3da5183b8daf63686d128cde[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Nov 21 19:39:31 2019 +0100

    update smpl-treasury with more logic

[33mcommit 3dbeb8051c62c644b269e230d9adb2935de6c4e1[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Nov 20 19:16:35 2019 +0100

    stash when debugging (considering adding an informal changelog for better communication moving forward)

[33mcommit 89de89d9958145956a50bfb7ff49b0104517f56d[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Wed Nov 20 12:36:33 2019 -0500

    Change import in child-trie module (#90)
    
    * Braindump.
    
    * attempt
    
    * x
    
    * correct file name 🤦
    
    * syntax
    
    * x
    
    * x
    
    * confirm build breaks
    
    * Check second module
    
    * indentation
    
    * all `run` items start from repo directory?
    
    * Check lots more modules
    
    * Even more modules
    
    * Refactor to use virtual workspace
    
    * x
    
    * Also check runtimes and node.
    
    * Only check modules
    
    * Add genesis modules
    
    * Add runtimes
    
    * Add node (this is where it all went wrong last time)
    
    * Remove node, but add recently-revised child tree.
    
    * Fix spelling mistake
    
    * Disable child-trie; It's not ready.
    
    * Import CHILD_STORAGE_KEY_PREFIX from consistent upstream commit.

[33mcommit 05856d5a8cd6c40b3dc7106388264c8bb7c8153b[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Nov 20 18:29:57 2019 +0100

    init externalities section

[33mcommit af5b2a0829bc35fd9043b34671286b8c2ef35910[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Nov 20 18:28:18 2019 +0100

    delete treasury and token sections, start changing smpl-treasury to be more featureful

[33mcommit 3f1e1248fdfd876ac237fc72d99d07f8d14f0b12[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Wed Nov 20 11:10:22 2019 -0500

    fix signature of Currency::transfer method in two modules (#88)
    
    * Tabs to spaces
    
    * Remove unused ReservableCurrency Trait bound
    
    * Update calls to `transfer` to match signature
    
    * Fix Reservable Currency
    
    * Pin dependencies in genesis module.

[33mcommit 0e15d005e491bb45e44db7cbb3a9bd780fa17d95[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 19 20:37:31 2019 +0100

    update SUMMARY.md according to proposed reorg

[33mcommit b6908b09c1ffb864963805c73aa6c66e5d154fba[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 19 18:08:08 2019 +0100

    init own testing section

[33mcommit 3db43f650c4b35c811e9fa84c221c54c93391c5d[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 19 15:06:51 2019 +0100

    update writing from Runtime to TestRuntime

[33mcommit f998738883697f51f8862d284ba365789ed315c2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 19 00:19:14 2019 +0100

    tps

[33mcommit 4db44fd41e5a9a17e9f2169962fb7e26a3346555[m
Merge: f0f09d2 742685a
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Nov 19 00:03:46 2019 +0100

    Merge pull request #85 from substrate-developer-hub/amar-finish-tests
    
    continue module testing

[33mcommit 742685a6effe2ef5d10869b0a9679c1ced0aac03[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 19 00:03:13 2019 +0100

    nit

[33mcommit 0f251e71d8c56ebbbdda563b8a6d467559023953[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 19 00:02:04 2019 +0100

    push to merge

[33mcommit 59744f44a0131b7337e8816810c2d971dc888527[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 23:12:00 2019 +0100

    change all mock  to

[33mcommit 55433e6c8503dc16066de0d9637c0552bb6d2823[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Nov 18 22:04:03 2019 +0100

    Update src/testing/mock.md
    
    Co-Authored-By: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>

[33mcommit 89cbc18f9d99f238d20056dc8ed8b92a5b64b065[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Nov 18 22:03:13 2019 +0100

    Update src/testing/mock.md
    
    Co-Authored-By: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>

[33mcommit dc62b4172f3478b82d94b96d07e8cd57e505901c[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Nov 18 22:03:03 2019 +0100

    Update src/testing/mock.md
    
    Co-Authored-By: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>

[33mcommit 28d52cc516f702ada0afe177f1df76ec08f7af9c[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Nov 18 22:02:52 2019 +0100

    Update src/testing/mock.md
    
    Co-Authored-By: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>

[33mcommit fb2fef950a2745c49ed774c28d5b5f11224b726b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Nov 18 22:02:13 2019 +0100

    Update kitchen/modules/storage-cache/src/lib.rs

[33mcommit e07606cc2573a57cf8cbe284a6b1f8a94e5365bb[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 16:32:29 2019 +0100

    use crate::Event instead of use super::super::*

[33mcommit 23ba237bf7a86e7da4e1839326d52ff9c04c6fc4[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 16:02:41 2019 +0100

    module constants with get syntax, tested

[33mcommit 17cb97636c44a81493b1c6d987d831b742deabd6[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 15:24:22 2019 +0100

    vecset tests

[33mcommit eb9b4c8bf6d66d7ef35405065c2064dd9fe20a49[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 14:48:11 2019 +0100

    storage cache tests

[33mcommit 8aaa5298e1d20e4be4958122083bc8603e461b1f[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 13:30:28 2019 +0100

    double_map tests

[33mcommit 1a33a6f58b17c0f8c9c0d3148c2a2ed28e4332d9[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 02:58:46 2019 +0100

    cover linked_map methods with tests

[33mcommit d2465dd9a992ac8fd2693e1fc152a0e2813bd64d[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 01:52:59 2019 +0100

    fix some messy typos and add links

[33mcommit a4f95827608926d347e8d706e04e17d06deb541a[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 01:45:30 2019 +0100

    cargo fmt on linked-map, add link, delete unnecessary word

[33mcommit fa4f8e23ae692c37501771b28ca5e94742d2b75e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 18 01:33:51 2019 +0100

    add linked map recipe, docs for mock runtime, clean some other tests

[33mcommit e175d9ea101c985ef2a2423b19a57dd1d2a8d6f1[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 19:12:50 2019 +0100

    run cargo fmt on a few modules, add test for simple-map

[33mcommit 4781f39196a5297e13000b850b47bf1af087343a[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 18:03:12 2019 +0100

    tested single value storage

[33mcommit ee42effa81056e11875ff3149c58233384877375[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 17:27:27 2019 +0100

    test generic event

[33mcommit 024c9b5da1874db490f732f9130bc7063ce3b407[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 16:44:42 2019 +0100

    simple event

[33mcommit 1e0f9f3fa7b7c99c06f9097a075e4c950f088356[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 16:30:28 2019 +0100

    better adding machine test

[33mcommit 316233794c8f8afd546f13bcb78cd1134fe5dd5e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 13:07:23 2019 +0100

    example of event testing in adding-machine

[33mcommit f0f09d214f4d8a069ba400347322b8d02ef5459b[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 12:29:33 2019 +0100

    copy summary.md

[33mcommit 0755ade79c951d60b96bc67bedfc54995ec28309[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 12:28:58 2019 +0100

    add back instantiables after preemptive merge

[33mcommit cc4d48d96036038e14514725dde41dc13c1c2303[m
Merge: c415a81 e7e72ae
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun Nov 17 12:20:40 2019 +0100

    Merge pull request #81 from substrate-developer-hub/unit-tests
    
    unit testing for some modules

[33mcommit e7e72ae7b62067bee63cde1d8ccf0b25c1606263[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 12:19:16 2019 +0100

    clean up before merge

[33mcommit 65ca6be14f7245647c07b9f032e6973dff6ce510[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 12:16:52 2019 +0100

    add back the pages lost in previous merges :/

[33mcommit 4c97f4c4fb93202fe6a2b8a036426b9e248a5cfa[m
Merge: bfd11fc c415a81
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun Nov 17 12:09:21 2019 +0100

    Merge branch 'master' into unit-tests

[33mcommit bfd11fced587c720fb8c37d26d18b32bef06f830[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Nov 17 11:58:59 2019 +0100

    clean up hello-substrate, finished with 2 tests

[33mcommit c415a81ce152b7dab5ec630fdb92f9a8c5a33eb3[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Fri Nov 15 14:10:00 2019 -0500

    Instantiable examples (#82)
    
    * Write 2 instantiable modules, include in runtime, port writeup from devhub, fix upstream commit.
    
    * Cleanup silly mistakes, oversights, merge conflicts.
    
    * fixed super-runtime default-instance
    
    * Update src/storage/instantiable.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update kitchen/Cargo.toml
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update kitchen/runtimes/super-runtime/src/lib.rs
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update src/storage/instantiable.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update src/storage/instantiable.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Anchor for default instance

[33mcommit 97cacfe6c54d3e1b7633607fedc9172130338919[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Nov 14 21:15:32 2019 +0100

    add tests for hello-substrate, init mock runtime for testing doc

[33mcommit 784cba5cde68ff30699496bd8379f93a58806772[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Nov 14 16:03:07 2019 +0100

    update hello substrate runtime logic

[33mcommit c0ca32cb62c74a436f42bbd92e4c7ae8b98f63bb[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Nov 14 15:35:44 2019 +0100

    init

[33mcommit 8756871d4a6e67b4fea6378c9f54a55a248b717a[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Nov 14 03:23:42 2019 +0100

    fix table of contents mistake

[33mcommit f1f2d2986f3133796e198755f47e48a2d737f731[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Wed Nov 13 14:37:37 2019 +0100

    Update runtimes to match upstream (#80)
    
    * Update super runtime
    
    * Fix weight-fee-runtime
    
    * Fix outer node

[33mcommit 3d87f76825945600784e62dc6fedf7276b59240a[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 12 22:04:58 2019 +0100

    fix the summary

[33mcommit 859d48ec49bdb0dd69709dc295cef2d03c23555e[m
Merge: 990a9ac 32abf9f
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 12 21:54:43 2019 +0100

    fix

[33mcommit 32abf9f27f0cc5aafa32531200b72bcd2fb7c2a2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 12 21:49:07 2019 +0100

    rename, add doc comments, delete any mention of inclusion proofs

[33mcommit 990a9acc0bb0d006232f05987d9c128f9b52d67a[m
Merge: df37d7e 4097ca7
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Nov 12 18:52:33 2019 +0100

    Merge pull request #78 from substrate-developer-hub/clean-up
    
    clean up

[33mcommit 4097ca7d5100d1f4041a25b1f4321537b70b23f1[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 12 18:50:11 2019 +0100

    more deletions

[33mcommit 1bcf892ee41364cfdb83a7761e9646f6a9602629[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 12 18:44:39 2019 +0100

    more clean up

[33mcommit 60ab4ff6b8a31c9cfe19dd34d170ff8cd3ce8f7b[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Nov 12 18:39:55 2019 +0100

    clean up

[33mcommit df37d7ecb79d53654d2c29e58ba8deb5b01d3a9f[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Sun Nov 10 18:19:01 2019 +0100

    (Merge) Update decl_storage! calls to use fn in getter declaration

[33mcommit 31a27805f1c8eb6fa2ed6758bc530a8dee186382[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Nov 4 02:17:42 2019 +0100

    change name

[33mcommit 058a416fdf9383356637a04ac4ac3268cb469430[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Sat Nov 2 18:18:09 2019 -0700

    Update kitchen/modules/child-trie/src/lib.rs
    
    Co-Authored-By: Kian Paimani <5588131+kianenigma@users.noreply.github.com>

[33mcommit 6027681df1b6291954997f295f1294e870ead17c[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Sat Nov 2 10:05:09 2019 -0700

    Transaction Weights (#62)
    
    * Brainstorm compiles.
    
    * Module fleshed out completely.
    
    * Runtime demonstrating WeightToFee
    
    * One more weight example.
    
    * line width to 100.
    
    * Put some text in recipes. Will be nicer after https://github.com/substrate-developer-hub/recipes/issues/43
    
    * Complete links to devhub.
    
    * Revert accidental change to weight module which broke compile.
    
    * Update kitchen/modules/weights/src/lib.rs
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Update src/design/econsecurity.md
    
    Co-Authored-By: Amar Singh <asinghchrony@protonmail.com>
    
    * Correct example WeightToFee = ConvertInto.
    
    * Update src/design/econsecurity.md
    
    Co-Authored-By: Kian Paimani <5588131+kianenigma@users.noreply.github.com>
    
    * Add Kian's suggested warning.
    
    * cleanup formatting characters
    
    * Add Amar's suggested warning.

[33mcommit c10fe261c19313d65e91ee6cc97b91bc0abbec07[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Nov 1 13:00:30 2019 +0100

    Apply suggestions from code review

[33mcommit 9f0ba4ea9d49138c80aedc82b336aef8f60b0e4e[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Nov 1 12:59:47 2019 +0100

    Update src/storage/childtries.md

[33mcommit 18ce2f1079102b82a1ad58c2970e7ac82cfb32f7[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Nov 1 12:57:56 2019 +0100

    Update src/storage/childtries.md

[33mcommit d70828faa40b403751de55307487269d50142032[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Nov 1 12:56:45 2019 +0100

    Update src/storage/childtries.md

[33mcommit a496749458dab8bf54d62a0ba336b6938c89e605[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Nov 1 12:48:40 2019 +0100

    Update src/storage/childtries.md

[33mcommit 2d2cd8228416f2bb9cd5d2d8a565282db158b843[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Nov 1 12:47:51 2019 +0100

    Update src/storage/childtries.md

[33mcommit 4f0e96deeadbf6423b1467d658b90c1c33c5ecb5[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Oct 31 03:19:04 2019 +0100

    refmt

[33mcommit 29e327445ccdf8796ee439eb48db32bf56271a53[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Oct 31 03:14:18 2019 +0100

    add page in book

[33mcommit fa02e8abef8b8cbd68f32f28d265e02459cb5a12[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Oct 30 16:50:00 2019 +0100

    make minimal example, more minimal

[33mcommit 4485eeb9a36451bbfadc036550a20d61df184168[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Oct 29 17:26:36 2019 +0100

    added smpl-crowdfund

[33mcommit eec371169a502c139302c71130153bb7dc7ea8c0[m
Merge: 5af55fb 1de38c4
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Oct 28 15:08:29 2019 +0100

    Merge pull request #74 from substrate-developer-hub/ch3-for-69
    
    Breaking Up Ch3

[33mcommit 1de38c43fc08df7a747016b619e5bb76cfa3c23b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun Oct 27 01:52:18 2019 +0200

    Update src/base/setup.md
    
    Co-Authored-By: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>

[33mcommit 1e95078282560916148375d0e2d872131d349e1f[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Oct 26 19:06:45 2019 +0200

    reference polkadot-js docs for now

[33mcommit 663feb73f9f59476bc51724901c5eb7b18811ce2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Oct 26 19:01:29 2019 +0200

    add link to 3.2

[33mcommit 0dbfcc7dd0bbc48ff8bb0dd5da5bf96940f0f426[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Oct 26 17:04:53 2019 +0200

    reformat chapter 3

[33mcommit 5af55fbf50de71fff027bc5f4df41a07b051fe0c[m
Merge: ec8c53d efe9f09
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sat Oct 26 00:37:19 2019 +0200

    Merge pull request #73 from substrate-developer-hub/joshy-fix-random
    
    Remove partial function signature to fix build.

[33mcommit efe9f0900905bd6ee0d89240d5871982e4cd32df[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Fri Oct 25 17:25:33 2019 -0400

    Remove partial function signature to fix build.

[33mcommit ec8c53d01f1888bbb50b3acc466f2f2967c4b663[m
Merge: 5375b62 a206b59
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Oct 23 14:12:12 2019 +0900

    Merge pull request #70 from substrate-developer-hub/joshy-runtime-rename
    
    Rename super-node-runtime to super-runtime

[33mcommit a206b5905d64b90c38ebfb0289a3920a17de53cc[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Tue Oct 22 11:18:19 2019 -0400

    Rename super-node-runtime to super-runtime

[33mcommit 5375b62b5d1e4988fd72f609c6b7bbb08b5e0449[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Tue Oct 22 10:47:29 2019 -0400

    Update for first round of transaction payment changes. More coming soon. (#68)
    
    * Update for first round of transaction payment changes. More coming soon.
    
    * Update node to compile

[33mcommit 448784be7b02c18a865b0ede058bd67da4a42208[m
Merge: 61ab629 40bab4e
Author: Jimmy Chu <jimmy@parity.io>
Date:   Tue Oct 22 20:03:12 2019 +0800

    Merge pull request #66 from jimmychu0807/jimmy/installation-rewrite
    
    `Setup` Chapter Update

[33mcommit 40bab4e3d9619d24432bd546a0ec38b8117b624d[m
Author: Jimmy Chu <jimmychu0807@gmail.com>
Date:   Tue Oct 22 11:34:05 2019 +0800

    Updated based on Joshy's feedback

[33mcommit fdc75def3e2b45ef7448e20ab216975167367b3e[m
Author: Jimmy Chu <jimmychu0807@gmail.com>
Date:   Mon Oct 21 17:13:04 2019 +0800

    Update node and blockchain name to `Kitchen Node`

[33mcommit 2aecbfb1a590e62152f3620863d04d095e49bcd8[m
Author: Jimmy Chu <jimmychu0807@gmail.com>
Date:   Mon Oct 21 17:06:20 2019 +0800

    Updated setup.md

[33mcommit 61ab6290be78e80aa890406bfbabbc6b76de163c[m
Merge: 02635d7 cc49c74
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Oct 21 17:42:34 2019 +0900

    Merge pull request #65 from jimmychu0807/jimmy/installation-rewrite
    
    Update kitchen to run with substrate master branch

[33mcommit 02635d7ef3ed3f90e6f094579bf04a3134620557[m
Merge: 1badd1b 6404d5b
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Oct 21 17:38:58 2019 +0900

    Merge pull request #64 from substrate-developer-hub/joshy-weird-babe
    
    Change code so it compiles, but doesn't make sense.

[33mcommit cc49c74b62142adadc8855f21d057c95092d21da[m
Author: Jimmy Chu <jimmychu0807@gmail.com>
Date:   Sun Oct 20 19:13:42 2019 +0800

    Update kitchen to run with substrate master branch
    
    Using master branch latest update of RandomnessCollectiveFlip::random_seed()

[33mcommit 6404d5b98ab3c96a645ae2ea731e86e083a0a7a2[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Sat Oct 19 02:08:24 2019 -0400

    Change code so it compiles, but doesn't make sense.

[33mcommit 4f74d88f6cb2bc91264d4cf9473d714b03de9b79[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Oct 15 11:45:36 2019 +0900

    fixes based on review comments

[33mcommit ed7c025663a713e226696348eaf6c5f9c156e28b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Oct 15 04:40:29 2019 +0200

    Update kitchen/modules/child-trie/src/lib.rs
    
    Co-Authored-By: Kian Paimani <5588131+kianenigma@users.noreply.github.com>

[33mcommit acb720cf1a1dae90a16d90420a6ad6e1c0a7e1b0[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Oct 15 04:40:22 2019 +0200

    Update kitchen/modules/child-trie/src/lib.rs
    
    Co-Authored-By: Kian Paimani <5588131+kianenigma@users.noreply.github.com>

[33mcommit 93070c5a69efd411996a963949cd9d7e3660c58a[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Oct 13 19:09:06 2019 +0900

    move notes to open pr

[33mcommit 8d1f2fcfded1a236302abe3d1613c609fd85a56b[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Oct 13 19:01:03 2019 +0900

    got compilation on minimal example

[33mcommit 1badd1b011dabb1a1c9f460079c6cf7d66a6e016[m
Merge: dc9071d 095a591
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Oct 11 10:57:38 2019 +0200

    Merge pull request #59 from jimmychu0807/master
    
    Removed extra word and added a todo on dead link

[33mcommit 095a591ba9244a9150f2b0be18c77c6598f222a7[m
Author: Jimmy Chu <jimmychu0807@gmail.com>
Date:   Fri Oct 11 13:02:47 2019 +0800

    Removed extra word and added a todo on dead link

[33mcommit dc9071d2cba987b7f6b05d88c2480b19866ec0b2[m
Author: Joshy Orndorff <JoshOrndorff@users.noreply.github.com>
Date:   Fri Oct 4 10:39:55 2019 -0800

    Include first runtime and node wrapper in kitchen (#52)
    
    * Super Node Runtime compiles
    
    * Basic example of default config.
    
    * One crate per config. In hopes of combining configs with runtimes later.
    
    * fix some typos
    
    * add basic-token config, update some deps
    
    * check-membership
    
    * double-map
    
    * linked-map
    
    * module constants with Get
    
    * schedule on finalize
    
    * simple-map
    
    * add simple events and storage; order existing from easy to hard
    
    * storage-cache example
    
    * push to merge
    
    * Add wrapper node to kitchen

[33mcommit b92ab8576198fbb843d49e7b28154fdc5b508aa3[m
Merge: 9a915e5 ca8b50a
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Oct 3 11:23:33 2019 +0200

    Merge pull request #50 from substrate-developer-hub/joshy-dsstore
    
    Remove all .DS_Store files.

[33mcommit ca8b50a84fffd23ef4204c66c209fa8464bb3482[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Oct 3 11:23:09 2019 +0200

    Update .gitignore

[33mcommit 9a915e5b06e3d545e2e989709504e09cd51c30d1[m
Merge: 18d901c 4c6f3c5
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Oct 2 18:26:23 2019 +0200

    Merge pull request #51 from substrate-developer-hub/joshy-links
    
    Update module-template and package links to devhub.

[33mcommit 18d901ccae9f12347c30be2d9cd583470a162ef2[m
Merge: 4677ed8 f97799f
Author: Shawn Tabrizi <shawntabrizi@gmail.com>
Date:   Wed Oct 2 17:45:02 2019 +0200

    Merge pull request #54 from substrate-developer-hub/ga
    
    Fix: mdbook does not support nested script.

[33mcommit f97799f8abc0785e427cb6064dbf4ec744fcae21[m
Author: Kaichao Sun <kaichaosuna@gmail.com>
Date:   Wed Oct 2 23:33:39 2019 +0800

    Fix: mdbook does not support nested script.

[33mcommit 4677ed8aab542755aa6d6387202a0d3dd347f956[m
Merge: 44f9dd5 94a1c34
Author: Shawn Tabrizi <shawntabrizi@gmail.com>
Date:   Wed Oct 2 17:23:49 2019 +0200

    Merge pull request #53 from substrate-developer-hub/ga
    
    Add google analytics and klaro

[33mcommit 94a1c341ab8a4a51c935bebbccf525557c2195af[m
Author: Kaichao Sun <kaichaosuna@gmail.com>
Date:   Wed Oct 2 23:18:00 2019 +0800

    Add google analytics and klaro

[33mcommit 4c6f3c55e087438747e5d46cee5e1ab272221149[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Tue Oct 1 14:31:10 2019 -0800

    Update module-template and package links to devhub.

[33mcommit 4a9230c81807c95b988aff2ef70cde20c146cb89[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Tue Oct 1 14:10:19 2019 -0800

    Remove all .DS_Store files.

[33mcommit 44f9dd545acac2f43bd2c2a48f49456d3f9a7801[m
Merge: 0e78382 61252fd
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Sep 30 15:38:48 2019 +0200

    Merge pull request #42 from substrate-developer-hub/some-structure
    
    linking better to existing docs/tutorials

[33mcommit 61252fdf34ffd14886465fcccf87041411583b1a[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 30 15:32:43 2019 +0200

    push to merge

[33mcommit 9b8f164c2b3d127611ecbb0af8304b02734e9d11[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 30 15:02:24 2019 +0200

    prepare to merge, next stage build

[33mcommit b1a7cb31eea72c66111be49f5e98daf7cd56c22e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 30 12:10:51 2019 +0200

    lots of changes, not ready to merge

[33mcommit feb62e6ce00b4310c9c88b9d515b7eae953b1129[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 30 02:27:37 2019 +0200

    writing for double map recipe

[33mcommit df8f8a5b2425055f44ca85b14128fe766c663f3e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 30 01:20:06 2019 +0200

    set storage and iteration

[33mcommit f5ea83f9553e4ddcbec97ae73ef601fa8226d7f7[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sun Sep 29 17:42:46 2019 +0200

    add storage cache recipe writing

[33mcommit e3a98e87673f8ccf8b9a413b26c64c3ebc1dfd3c[m
Merge: 6fe81f3 907b460
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Sep 28 15:49:22 2019 +0200

    fix

[33mcommit 6fe81f3887f2e3b2f8318ce5ada91a1240465955[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Sat Sep 28 15:48:07 2019 +0200

    more changes to structure

[33mcommit ae4566bffff6188d99db762d611aee310f922876[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Fri Sep 27 12:14:22 2019 -0800

    Double maps require explicit second hasher.

[33mcommit 907b4603ae9e18e6aafb243d814b424e4ca28801[m
Merge: 0bfe5f8 e2a8326
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sat Sep 28 13:28:32 2019 +0200

    Merge pull request #44 from JoshOrndorff/joshy-double-map
    
    Double maps require explicit second hasher

[33mcommit e2a8326f78c4e2e7afe116e280801fb12b3c7948[m
Author: Joshy Orndorff <admin@joshyorndorff.com>
Date:   Fri Sep 27 12:14:22 2019 -0800

    Double maps require explicit second hasher.

[33mcommit 0bfe5f84234034856330a366f6d9bc6cac36c9d3[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 26 17:12:16 2019 +0200

    double-map example will not compile :(

[33mcommit 77129b9a0c3db478e53678665fb1eebaa39e3986[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 26 14:47:53 2019 +0200

    add storage cache example

[33mcommit fda1633e78db047af3cb670452fb6ce74265c4b4[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 26 13:57:02 2019 +0200

    append vs mutate for vec storage, linked map recipe

[33mcommit 0100846d60851148961ead02462666249db47b52[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Sep 25 14:47:12 2019 +0200

    simple map

[33mcommit c5877267810de63895a6aeab09cf0153c534f5f8[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Sep 25 13:22:33 2019 +0200

    minimal event examples

[33mcommit 34881666d1754798fb3ffcdcf4e377ae36517eb2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 23 16:02:13 2019 +0200

    fix recognition

[33mcommit 702f83fb8d71b408ef13c9fc9f0c745cfba9a485[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 23 14:42:19 2019 +0200

    update structure

[33mcommit 365ac0e1da9854b01540e71b95ff595f5bcc0175[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 23 14:20:26 2019 +0200

    add hello world example

[33mcommit a19c2d261f338675dcab9cb4a6387f4b84d3d0df[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Sep 23 11:18:43 2019 +0200

    add intro to rust recipe

[33mcommit 1a6162dd5fcc028806d2abd416ff92a6337e1b4b[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Sep 20 01:42:19 2019 +0200

    restructure kitchen

[33mcommit 6d8dfd47e1f7089c661a78968a0e3b22fbeba4d5[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Sep 20 01:38:08 2019 +0200

    init new structure

[33mcommit 0300e9a255e6c1695e51f2926603974eece52b63[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Sep 19 19:45:06 2019 +0200

    Update kitchen/random/src/lib.rs

[33mcommit 375df272623d52bb377b80b20844257701a15439[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 19 19:22:09 2019 +0200

    fix build, fix rr

[33mcommit 80378e067db5eb8bd95a2ab68ba3fc0677a99e67[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 19 18:49:50 2019 +0200

    update contribution guidelines to align with new doc guidelines

[33mcommit 0e783821287b98a2eefae80c9ce78d283794494d[m
Merge: a32549e a2acb36
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Sep 6 18:05:04 2019 +0200

    Merge pull request #38 from substrate-developer-hub/treasury
    
    treasury, rng, runtime hooks

[33mcommit a2acb367b3db3c78d4d6d160d8af0b8af89d1750[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Sep 6 17:57:35 2019 +0200

    fix broken links

[33mcommit f6e410d9422ea35af50b524841dbd77826691982[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Sep 6 17:46:28 2019 +0200

    fix broken links

[33mcommit 32e4db9c18effb2614f1f2bf9e6718e9e8fc16b7[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Sep 6 17:44:50 2019 +0200

    finish treasury recipe, consider merging

[33mcommit eeff977c6210556c1c22e29244820e4974d84077[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Sep 6 03:00:32 2019 +0200

    add scaffolding

[33mcommit 328741cec830ac9638d2964f8e0f4d47c5e7b799[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 5 17:04:10 2019 +0200

    init treasury, random, runtime hook

[33mcommit a32549e9e6286bc661168292b7d09a1f44983775[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 5 13:56:15 2019 +0200

    update randomness wip

[33mcommit 6b7babc3750a0104edeee1393816f02028ad1f23[m
Merge: 8665361 e176641
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Sep 5 04:29:11 2019 +0200

    Merge pull request #37 from substrate-developer-hub/collateral
    
    Currency Types and Locking Techniques

[33mcommit e176641f837c52ef26031e6fe94ad169e5987a2d[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 5 04:16:26 2019 +0200

    update package names

[33mcommit a66fe95d80c543a456199ad6fbc82c8426ebb36e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 5 04:12:18 2019 +0200

    clean up, finish collateral recipe

[33mcommit b8c6b4193817cbaab53f96b670d8fe848fa092d2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Sep 5 01:49:36 2019 +0200

    fixed collateral recipes in the kitchen

[33mcommit 866536131c48a4972aed9e108b4c23d26efccc5f[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Sep 3 13:38:04 2019 +0200

    fmt kitchen, fix broken link, add randomness draft

[33mcommit c936cfb661fcd363d871e553f7a921b2fe0ca298[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Aug 30 13:17:40 2019 +0200

    update readme

[33mcommit 5427bfd4fbc43801d3a4bc17391c85ccb55e254e[m
Merge: 2723c8a f92e765
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Aug 30 13:01:59 2019 +0200

    Merge pull request #30 from substrate-developer-hub/remodel-kitchen
    
    V2 Push

[33mcommit f92e765a334baca465d20c6ec2438b2463dd6eb2[m
Merge: dfbdc3c 2723c8a
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Aug 30 12:55:23 2019 +0200

    Merge branch 'master' into remodel-kitchen

[33mcommit dfbdc3c22a780973a6303d0251396ac7c139a65c[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Aug 30 12:44:41 2019 +0200

    push to merge

[33mcommit 57bcdcd34809171ecf145258b69d16d9a233a8c1[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Aug 30 10:32:42 2019 +0200

    update balance usage recipe

[33mcommit de1c22b2645587f7a52435cc709038ec369979f2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Aug 28 18:12:40 2019 +0200

    added configurable module constants and related docs

[33mcommit c76c1046e82bc1b070e6716276e14b040209849f[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Aug 27 19:01:27 2019 +0200

    finish positional reorg

[33mcommit 01d3a8d5a8ef1b5cb88844ba1258dcc1c3ef283e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Aug 27 16:26:53 2019 +0200

    restructure everything

[33mcommit 3d8a17da77c73bd9dcf8f8ba0e261f77809b35a2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Aug 27 14:50:10 2019 +0200

    permissioned methods recipe

[33mcommit c9acacff9d2dec1daf4965b0bace0d7ff86450d4[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Aug 27 12:52:00 2019 +0200

    finish writing blockchain event loop

[33mcommit 2a00f5ea23be7446b25783510129dff2b7f78f78[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Aug 26 18:11:19 2019 +0200

    compilation and added a few more recipes

[33mcommit d627b7a13b1e858ab6ef63c42e380ebfdb0af593[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Aug 26 17:36:37 2019 +0200

    add balances example

[33mcommit c7d4ebf5595d2cccde95fbc73d8abcdcadaed5d0[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Aug 26 16:50:16 2019 +0200

    added loop recipe for scheduling execution events

[33mcommit b5da01da995332292d65b48f92ab5d5694f7e357[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Aug 26 14:08:11 2019 +0200

    init

[33mcommit 2723c8a906ac9a81eea8290c2ad38eb3569f3e77[m
Merge: 50e2804 f1101d2
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Aug 5 11:55:30 2019 +0200

    Merge pull request #27 from substrate-developer-hub/resources
    
    Add substrate dev hub and references doc link in resources.

[33mcommit f1101d2830354d9b64c675b959d83b465243da19[m
Author: Kaichao Sun <kaichaosuna@gmail.com>
Date:   Sat Aug 3 14:30:36 2019 +0800

    Add substrate dev hub and references link in resources.

[33mcommit 50e280466967327d6e9d67618eaad764d409ecb7[m
Merge: 43fc924 fbabdd6
Author: Shawn Tabrizi <shawntabrizi@gmail.com>
Date:   Thu Jul 11 02:46:33 2019 +0200

    Merge pull request #25 from shawntabrizi/master
    
    Clean up, add node template

[33mcommit fbabdd6aedfa9e74e64a9cd407d722638c73f63e[m
Author: Shawn Tabrizi <shawntabrizi@gmail.com>
Date:   Thu Jul 11 01:35:49 2019 +0200

    Clean up, add node template

[33mcommit 43fc924e57b6840bf6e13992f1536881cd8f5ba1[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Jul 10 11:07:27 2019 +0200

    fix broken links #15

[33mcommit c4b85d771acf2dab500f718dcf39027b627443f4[m
Merge: c4eb6b8 5517a1f
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Jul 10 10:38:01 2019 +0200

    Merge pull request #21 from substrate-developer-hub/vpoin2
    
    Add Executable Recipes, Update Existing Code, Address #20

[33mcommit 5517a1fbd468b3e2dada7125c4333003e4f6b0cd[m
Merge: f53e494 c4eb6b8
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Jul 10 10:32:02 2019 +0200

    Merge branch 'master' into vpoin2

[33mcommit f53e49474a37a945b236a15dff7a0acf816782f9[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Jul 10 10:23:28 2019 +0200

    fix to merge

[33mcommit 87c5881935033bc9e8a35ad9d4c829f8931d4ce4[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Jul 10 10:22:08 2019 +0200

    clean up

[33mcommit d3cac38b24a8e2655cdcbb988d6973b2dc7008b2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Jul 10 10:21:02 2019 +0200

    push to master

[33mcommit 7afa472356e907f5430cf31cac1682ac1c68d11e[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Jul 10 09:54:29 2019 +0200

    address issue #20

[33mcommit 757b8e21643ee5c14c50532ab0ff0ada474fc032[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Jul 9 17:49:58 2019 +0200

    clean up

[33mcommit 6d7aa3fb02b98b76eeb2ea20dcfd81145f46506a[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Jul 9 14:12:21 2019 +0200

    finished storage

[33mcommit ae8aa2db67bc481881a601245536ddbacb9f82f2[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Jul 8 17:32:04 2019 +0200

    most of storage, phew, this is a nontrivial rewrite

[33mcommit 2da8cb0bf08d72a8a43f6ad45903257e6fd32028[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Mon Jul 8 15:21:58 2019 +0200

    add kitchen and kitchen/event

[33mcommit 3212293bce95e1bd4333d9181bebd3d6db3c7f84[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Jul 5 15:17:25 2019 +0200

    init executable space

[33mcommit 876b828ebfae76b7a0dfa3ae6495673b159fcd3b[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Jul 4 15:17:18 2019 +0200

    new plan, better organization

[33mcommit c4eb6b852c4befcaf98d175c0a53d9cab2dc9eb5[m
Merge: 3b8be55 7a713f6
Author: Shawn Tabrizi <shawntabrizi@gmail.com>
Date:   Sun Jun 30 20:08:15 2019 +0200

    Merge pull request #17 from vedant1811/patch-1
    
    Fix broken link

[33mcommit 7a713f6b1e86b878b2680f0b67e1283c4128b7e8[m
Author: Vedant Agarwala <mail@vedant-agarwala.com>
Date:   Mon Jul 1 01:10:34 2019 +0800

    Fix broken link

[33mcommit 1a4c42ab9cdb95513d70522b5995fe1254ff9569[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Jun 27 14:17:52 2019 +0200

    clean up and add soon

[33mcommit 472b9710426d80b768d230b84d11624f8dc9b203[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Wed Jun 26 15:02:58 2019 +0200

    update

[33mcommit 97982bc560519e84983f5909b96edd0585bd6422[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Jun 21 12:49:25 2019 +0200

    reorganization

[33mcommit 3b8be55fce07811e7a6c83b73189426dbd7c56d7[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Fri Jun 21 11:28:34 2019 +0200

    fix instruction

[33mcommit 8e1c1f0c6bdbcd8f8ca72cdacc1d331fe03417af[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Thu Jun 20 15:45:39 2019 +0200

    fix link

[33mcommit 6d22f380edde28e5d9ae6b15a0bc6aa6a6575a56[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Jun 18 11:17:11 2019 +0200

    fix link on CONTRIBUTING.md

[33mcommit 17e6b5a9c56077f38005846addad85231afa08d5[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Jun 18 11:09:20 2019 +0200

    fix links

[33mcommit 993bc22268088e4127e3f8e206e1b7d42fbc82b4[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Jun 18 00:39:51 2019 +0200

    fix readme link

[33mcommit 382865145fc1f500f7ab2a8655c335a257aca6fd[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Jun 18 00:38:54 2019 +0200

    small edits

[33mcommit 75eec7ee96e47ed41eb1944c5001c4f28dcead93[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Jun 18 00:25:36 2019 +0200

    fixing build

[33mcommit 4a094da42f05def8e4787d63b6f9dfdad13de467[m
Author: 4meta5 <asinghchrony@protonmail.com>
Date:   Tue Jun 18 00:12:09 2019 +0200

    update everything

[33mcommit 1cb16bed13b1d44ca368265e21a634d07c6c0d84[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 16 14:42:51 2019 +0100

    ssfl0x8675309

[33mcommit 7c28e84c7c4d1d67148f54f6cf1948acc0d96fb5[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 16 14:26:37 2019 +0100

    ssfl0x8675309

[33mcommit bc34401321b9a71893db800b4b14b7a88a86c418[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 16 14:12:49 2019 +0100

    ssfl0x8675309

[33mcommit 773e0f8a9fab376fc175068871e8f1e611ed5b7c[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 16 13:26:06 2019 +0100

    ssfl0x8675309

[33mcommit 5eb0dfc72f02c739075daafb7911ac314daeff45[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 16 02:43:05 2019 +0100

    ssfl0x8675309

[33mcommit d572c9d3a532d4de6a39400e85c5a6aac281fe4e[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 16 02:26:46 2019 +0100

    pretty much done...clean up tomorrow and post

[33mcommit 7b56a5c4a5ce77178deab5c3dd3a0c7a771d1a1a[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed May 15 18:19:34 2019 +0100

    aveiro

[33mcommit f48460bc6701d7ed7b4de96d4ee7aa81709bb992[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue May 14 12:48:37 2019 +0100

    ssfl0x8675309

[33mcommit 155c6a416946ffe77358d4dda483a4d7d1c72a24[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon May 13 18:34:02 2019 +0200

    ssfl0x8675309

[33mcommit da5b99f9af482733ee0b368c3d12457c252be35b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon May 13 13:02:55 2019 +0200

    ssfl0x8675309

[33mcommit 4b9c8503d380ae8573dd064f7a24e544d9b9eed9[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun May 12 22:42:03 2019 +0200

    ssfl0x8675309

[33mcommit 7106731063c820d4624355ba58e5a9ed7b277dc9[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun May 12 17:50:37 2019 +0000

    ssfl0x8675309

[33mcommit 5c9dcd614c0823fbad226497d0f757a84405dc0b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun May 12 17:33:16 2019 +0000

    ssfl0x8675309

[33mcommit 4e67282e86bff072772fe4eef0ffccf389ae3352[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun May 12 17:21:42 2019 +0000

    ssfl0x8675309

[33mcommit efd2cfc74189e891cbe6f04ecdb151537bdab547[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 9 03:02:01 2019 +0200

    ssfl0x8675309

[33mcommit 895ea2136cf60643a65a092bc7119c30319206d0[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 9 00:45:55 2019 +0200

    ssfl0x8675309

[33mcommit 75075e8b585c3c945e48e3bea33a3bc9b648930d[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed May 8 23:35:01 2019 +0200

    ssfl0x8675309

[33mcommit fc3aa9df3ae525ba22b9ba49a1ac7f50c1b18b64[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed May 8 21:46:50 2019 +0200

    ssfl0x8675309

[33mcommit 0d4e92a9a612821702eb2a072c8414a631e02264[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue May 7 21:50:04 2019 +0200

    ssfl0x8675309

[33mcommit 04080acdc5ba605e383fa9e74cd4676dc0b52edb[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue May 7 19:41:01 2019 +0200

    ssfl0x8675309

[33mcommit c81b7a36e061fa2d8f6b8d2ff007583f63ed1688[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue May 7 18:07:23 2019 +0200

    ssfl0x8675309

[33mcommit 8ef1ae01b4e6d50a99f36e806f0889bed23517a0[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue May 7 18:04:12 2019 +0200

    ssfl0x8675309

[33mcommit b93652c087d1f3d56264897613575e9bacb1a56a[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon May 6 12:10:43 2019 +0200

    ssfl0x8675309

[33mcommit 21364d7f073bed0fd2a662f70f1b3ad2c2e9ddd7[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon May 6 12:03:49 2019 +0200

    ssfl0x8675309

[33mcommit 56ae1962444bd4df35144ae2e139b54adb10fd1b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun May 5 14:03:53 2019 +0100

    ssfl0x8675309

[33mcommit d529792c401d7a689d7442b5255baa23bc13c321[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sat May 4 21:33:31 2019 +0100

    ssfl0x8675309

[33mcommit 4fdd17f30723060022024fd66fc435f962bd5311[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri May 3 12:45:15 2019 +0200

    ssfl0x8675309

[33mcommit 09a63ea8964cfc29c2f611feffb8c92d14d2ccf7[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri May 3 12:11:51 2019 +0200

    ssfl0x8675309

[33mcommit a4d388a786215b3ffb8c6c220353217e14a322e9[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri May 3 02:27:01 2019 +0200

    ssfl0x8675309

[33mcommit 1df94bf81e67bc9b7dd171ee100d86a691f8f89a[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri May 3 02:09:40 2019 +0200

    ssfl0x8675309

[33mcommit 5628f7ed0f888d4326532232fa437511ab67e03d[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri May 3 00:48:08 2019 +0200

    ssfl0x8675309

[33mcommit 79440d738c2f0a2f0c971623ed0674977e8234df[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu May 2 16:43:05 2019 +0200

    ssfl0x8675309

[33mcommit e12bbdbef02bd870a240ef129a0a03c9e7e10505[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed May 1 20:48:39 2019 +0200

    ssfl0x8675309

[33mcommit b9fe8ab2dd4dae6c0381e6ddc785dccdbb0f4a5f[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed May 1 15:49:42 2019 +0200

    ssfl0x8675309

[33mcommit a307931bb8cd89fd059e752ffdef83813f4eaa7f[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Apr 30 10:27:15 2019 +0200

    ssfl0x8675309

[33mcommit 95243957d88241a13e845e3ce470f68bdaf22bc6[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Apr 29 21:18:33 2019 +0200

    ssfl0x8675309

[33mcommit 2429d3c89b9d92856e8d0303dd27d04f3caf252b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Apr 29 15:41:25 2019 +0200

    ssfl0x8675309

[33mcommit bb22a2f8c3da2801d4b0a16dc1aada707d33ec82[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Apr 29 15:09:58 2019 +0200

    ssfl0x8675309

[33mcommit cd9218866c1e468f8dcafb855969e006da05f6be[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Apr 29 10:52:00 2019 +0200

    ssfl0x8675309

[33mcommit ba0ed876da2c73c7601a5a1cce552d8c76c7b282[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun Apr 28 22:22:55 2019 +0200

    ssfl0x8675309 && pit push -u origin master

[33mcommit 1d0265bbbbced0f2c8a9fd691995d013fd327fa3[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sat Apr 27 17:19:21 2019 +0200

    ssfl0x8675309 && pit push -u origin master

[33mcommit 4b4d206106a17a06a9735e13af27d31e82f1d0b7[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sat Apr 27 16:08:31 2019 +0200

    fix broken build

[33mcommit edc1bd7ad69c7ce05dc2aa014a21ee231011caec[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sat Apr 27 04:15:10 2019 +0200

    updating

[33mcommit c69e0befb129d435f601be60673dc95404e4f382[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Apr 17 14:17:07 2019 -0100

    trying to deploy to now

[33mcommit 8132897a86d0d0461779fcfa24c768d0ffed3bbe[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Apr 16 23:35:54 2019 -0100

    dploy on now

[33mcommit 233d11df8881b3f0cdb385c984c1f7e8dc493325[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Apr 16 23:29:04 2019 -0100

    update

[33mcommit 1b9cc0396068c1d2dad4f9d08a7bd45a4a6ea1d3[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Apr 16 22:25:57 2019 -0100

    added testing section

[33mcommit 2735403edea32668cc01ca90cf1859b3d8de400b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Apr 16 19:48:28 2019 -0100

    <before>

[33mcommit 2803168a6b48332a158fcf36d4a8962a53772f0a[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Apr 16 17:48:13 2019 -0100

    updated map sections

[33mcommit 47c4bb1e44a09e93dbdf283afda946fa33bfc4d2[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Apr 10 11:46:18 2019 +0200

    t.build()

[33mcommit f5f434d98c83e4e29a39e2bee1bb2ffc1f650eda[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sat Apr 6 23:03:51 2019 +0200

    some notes added

[33mcommit 7a5925c70ec39d86b907562d1aac8c300944d1ca[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Apr 4 11:55:39 2019 +0200

    commit

[33mcommit 4964809212c3f899c4c94c48d5f4c88bb46a997f[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Mar 25 01:45:27 2019 +0100

    cmit

[33mcommit abc5fbb4a9679548c86a36587e7129e1bf57ac10[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Mar 25 01:37:34 2019 +0100

    pls build

[33mcommit 3a3b1a40863897751ad665552444c10da31ee8d0[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Mar 12 15:49:43 2019 +0100

    ff

[33mcommit f6dd3386425e0affac224735a47b8d467cae0f5a[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Mar 12 15:43:26 2019 +0100

    another token, another try

[33mcommit 61aec4b42609b26b4de3372518d256922e0be586[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Mar 12 15:33:02 2019 +0100

    cmit

[33mcommit 4c3bcce509560808dd99915000ced45ec20d9bb0[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Mon Mar 11 00:55:36 2019 +0100

    running some tests

[33mcommit f725854a33249959a59994e85423477624cd8c52[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun Mar 10 21:59:08 2019 +0100

    Module Menu 2nite

[33mcommit 8a5e285bbc9ecd7f23971db0757120c8a1c0fe05[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Sun Mar 10 15:08:05 2019 +0100

    01110011

[33mcommit d81873ddb49f54d3a0716cea8861122c1f648836[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Fri Mar 8 10:56:10 2019 +0100

    fmt

[33mcommit 1873d203e7a5148a9b26f95047240210e4f3e92a[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Mar 7 15:47:06 2019 +0100

    fmt

[33mcommit bb128468d0f133f6cf82484e72d880a2ad39fa7a[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Mar 7 15:33:26 2019 +0100

    open source is important

[33mcommit e671c0c8295539758eb1593893e3b31c001ce8b9[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Thu Mar 7 12:27:12 2019 +0100

    cmit

[33mcommit 5248efac3a1174e501fd061cd0cfc36bb5ab0ded[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Wed Mar 6 16:47:44 2019 +0100

    cmit

[33mcommit 92c3786d95743748f8d0ac12dd630050d8bd3f27[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Mar 5 18:37:58 2019 +0100

    progress

[33mcommit a92cfecd2591c87e62455510ab254989ed08d1a1[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Mar 5 16:32:17 2019 +0100

    how many builds am I limited to?

[33mcommit d45e40a2d93691196dcf563baf19eb7f1af4e312[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Mar 5 14:05:55 2019 +0100

    try

[33mcommit 8f3350a7debbf3dfc406384ab6c3640f911a063b[m
Author: Amar Singh <asinghchrony@protonmail.com>
Date:   Tue Mar 5 13:54:56 2019 +0100

    setup
