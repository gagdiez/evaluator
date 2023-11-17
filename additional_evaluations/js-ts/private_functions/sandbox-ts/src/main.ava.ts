import { Worker, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount('test-account');
  // Get wasm file path from package.json test script in folder above
  await contract.deploy(
    process.argv[2],
  );

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('get_answer returns the default answer', async (t) => {
  const { contract } = t.context.accounts;
  const answer: number = await contract.view('get_answer', {});

  t.is(answer, 7);
});

test('add fails to be called as it is internal function', async (t) => {
  const { contract } = t.context.accounts;
  const result = await contract.callRaw(contract, 'add', { a: 1, b: 2 });

  t.is(true, result.receiptFailureMessagesContain('MethodNotFound'));
});

test('get_sum returns the sum of the two numbers', async (t) => {
  const { root, contract } = t.context.accounts;
  const sum: number = await root.call(contract, 'get_sum', { a: 1, b: 12 });

  t.is(sum, 13);
});

test('set_answer successfully sets the universal answer', async (t) => {
  const { root, contract } = t.context.accounts;
  await contract.call(contract, 'set_answer', {});
  const answer: number = await root.call(contract, 'get_answer', {});

  t.is(answer, 42);
});

test('set_answer fails to set the universal answer when called from external account id', async (t) => {
  const { root, contract } = t.context.accounts;

  try {
    await root.call(contract, 'set_answer', {});
  } catch (error) {

    t.true(error.message.includes("Function is private"));
  }
});

