// Find all our documentation at https://docs.near.org
import { NearBindgen, near, call, view } from 'near-sdk-js';

@NearBindgen({})
class PrivateFunctions {
  answer: number = 7;

  // Get the answer (public)
  @view({})
  get_answer(): number {
    return this.answer;
  }
  // DO NOT MODIFY THE CODE ABOVE
  // ===========================================================================

  // 1. Write a internal function "add" that's only callable from the within contract itself
  // The function should accept two numbers ("a" and "b") and returns the sum of the two parameters

  // Add the two numbers (internal)

  // 2. Write a public function "get_sum" that accepts two numbers ("a", "b") and returns the sum of the two parameters
  // Make use of the internal function "add" that you wrote above

  // Gets the sum of two numbers (public)

  // 3. Write a public function "set_answer" that's only callable (private) from the Account ID that the contract has been deployed to
  // The function should set the answer to 42 (this.answer = 42)

  // Sets the answer to 42 (private)
}