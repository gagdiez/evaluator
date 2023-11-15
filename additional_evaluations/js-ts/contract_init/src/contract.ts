// Find all our documentation at https://docs.near.org
import { NearBindgen, near, call, view, initialize } from 'near-sdk-js';

@NearBindgen({})
class HelloNear {
  greeting: string = "Hello";
  // DO NOT MODIFY THE CODE ABOVE
  // ===========================================================================

  // Write your initialization function `init` here


  // DO NOT MODIFY THE CODE BELLOW
  // ===========================================================================
  @view({}) // This method is read-only and can be called for free
  get_greeting(): string {
    return this.greeting;
  }

  @call({}) // This method changes the state, for which it cost gas
  set_greeting({ greeting }: { greeting: string }): void {
    near.log(`Saving greeting ${greeting}`);
    this.greeting = greeting;
  }
}