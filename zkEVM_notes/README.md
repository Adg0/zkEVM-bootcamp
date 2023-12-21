# What are zkEVMs?

zkEVM is a virtual machine designed and developed to emulate the EVM by recreating all existing EVM opcodes.

## Understanding zkEVM

1. Understand VM
2. Understand zkVM: zk aspect added to the VM
3. Understand zkEVM: a zkVM where the VM is an EVM
4. zkEVM running on L2

## Abbreviations:

- VM: virtual machines
- EVM: Ethereum Virtual machine
- SM: State machine
- IR: Intermediate Representation
- LLVM: Low level virtual machine, is a compiler framework for programmatically generating machine-native code.
- ISA: Instruction Set Architecture
- RISC-V: Reduced Instruction Set Computer, fifth generation
- DSL: Domain specific Language
- R1CS: Rank-1 Constraint System
- PP: Public Parameter
- PK: Proving key
- VK: Verifying key
- SMT: Sparse Merkel Tree
- PCS: Polynomial Commitment scheme

## Definition:

- ISA is abstract model of how the CPU is controlled by software. Three types:
	- Register based
	- Stack based: EVM is stack based
	- Accumulator based

## zkEVM workflow

The general workflow for an zkEVM would be 

1. Receive a transaction 
2. Execute the relevant bytecode 
3. Make state changes and transaction receipts 
4. Using the zkEVM circuits with the execution trace as input produce a proof of correct execution 
5. Aggregate proofs for a bundle of transactions and submit them to L1
6. Submit data to the appropriate data availability layer.

## zkProcess

1. Arithmetisation
	- Transform DSL to a circuit
	- Transform to polynomials
	- Prover <-> Verifier interaction
	- Make non-interactive
