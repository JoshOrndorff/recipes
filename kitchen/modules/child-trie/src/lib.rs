// FROM ROB: What we're doing there is storing the root of another trie under a key in the 
// main trie. Since keys are typically hashed in the main trie, it's a fairly 
// convenient (although not the only) way to make sure that inserting many elements 
// under a single mapping does not make your typical lookup path for unrelated elements longer