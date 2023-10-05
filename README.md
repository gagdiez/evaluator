NEAR Evaluator
==============

> This is a work in progress

This project aims to create a tool that automatically evaluates students on NEAR smart contract development. It was concieved to be used within a larger educational project, in which students follow a self-pased course, and can automatically evaluate their gain knowledge.

From a technical perspective, `NEAR Evaluator` is a smart contract that automatically evaluates other smart contracts. It works by making a cross contract calls to the contract being evaluated, and checking the data it returns.

On passing all evaluations, students will be able to claim a `NEAR Certified Developer` NFT.

---

## Evaluation
To be evaluated, the student needs to first call the `register` method, to register the account being evaluated. Afterwards, we will expect the student to call all methods using always the registered account, and to deploy all smart contracts in a sub-account of the registered account.

### [1. Hello NEAR](contract/src/eval_hello.rs)
Here we evaluate that the student knows how to deploy a simple smart contract. 

```rs
  evaluate_hello_near(contract_account_id: AccountId)
```

The contract makes a batch call to `contract_account_id`, calling `set_greeting` with a random string and `get_greeting`. The expected result of `get_greeting` is the random string that was set.  


### [2. Guest Book](contract/src/eval_guestbook.rs)
Here we evaluate that the student knows how to store an array of messages on a contract.

```rs
  evaluate_guestbook(contract_account_id: AccountId)
```

The contract makes a batch call to `contract_account_id`, calling `add_message` twice, and then `last_messages({last: 2})`. The contract being evaluated is expected to return the two messages that were added.  

### [3. Complex Datatypes](TBD)
Here we evaluate that the student knows how to handle types such as `AccountId`, `U64`, and `Objects`.

```rs
  evaluate_datatypes(contract_account_id: AccountId)
```

We make a batch call to `set_data` and `get_data`. We expect `get_data` to return a new object containing all the complex data given to `set_data`.

### [4. Explicit Init](TBD)
Here we evaluate that the student knows how to init a contract.

```rs
  evaluate_explicit_init(contract_account_id: AccountId)
```

We make a call to `get_greeting`, which we expect to fail, and then a batch call with `init` and `get_greeting`.

---

#### Progress Checklist
- [x] Evaluate a `hello world` contract
- [x] Evaluate a `guestbook` contract
- [ ] Evaluate the input and output of complex types / objects
- [ ] Evaluate an explicit init
- [ ] Evaluate private and public methods
- [ ] Evaluate the use of collections
- [ ] Evaluate basic actions
- [ ] Evaluate cross-contract calls
- [ ] Implement a Simple BOS Frontend

Long Term Planning
====
[ ] Evaluate factories
[ ] NEP Evaluator for NFT & FT Contracts