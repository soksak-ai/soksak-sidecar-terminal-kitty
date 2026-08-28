# Build dependency transactions

`build-dependencies.json` owns the Kitty provider repository, exact commit, Python version, target
set, and output tree. The Makefile contains commands only.

Preparation keys source and build caches by target and source commit. If the current receipt
matches the declaration, it is reused. A receipt failure for the same declared source is corruption
and is refused. A different valid source commit is replaced only after a new transaction has built
and validated the complete SDK and receipt.

Replacement moves the previous target and receipt into the transaction, installs the validated
pair, validates it at the final path, then removes the previous pair. A failed final validation
restores the previous pair. Read-only SDK trees are made owner-writable only after they have left the
active path. A process-owned lock serializes preparation; a dead owner is reclaimed on the next run.

The test SDK copied beside Rust test binaries follows the same next/previous rename rule. It is not
modified in place.
