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
  const allowed = await root.createSubAccount('allowed');
  // Get wasm file path from package.json test script in folder above
  await contract.deploy(
    process.argv[2],
  );

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, allowed, contract };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log('Failed to stop the Sandbox:', error);
  });
});

test('check_predecessor returns true when called by specific accountId', async (t) => {
  const { allowed, contract } = t.context.accounts;
  const result = await allowed.call(contract, 'check_predecessor', {});

  t.is(result, true);
});


test('get_current_block_timestamp returns correct block', async (t) => {

  const { root, contract } = t.context.accounts;


  const result = await root.view('get_current_block_timestamp', {});

  const greeting: string = await contract.view('get_greeting', {});

  t.is(greeting, 'Howdy');
});