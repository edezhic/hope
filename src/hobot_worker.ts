const core: Worker = self as unknown as Worker;

async function build_script(title: string, body: string) {
  const { build } = await import('../hobot/pkg');
  const [tokens, graph] = build(title, body);
  core.postMessage({
    type: 'tokens&graph',
    tokens,
    graph,
  });
}

async function send_test() {
  const { get_test } = await import('../hobot/pkg');
  core.postMessage({
    type: 'test',
    tests: get_test(),
  });
}

core.addEventListener('message', (evt) => {
  switch (evt.data.type) {
    case 'build':
      build_script(evt.data.title, evt.data.body);
      return;
    case 'get_test':
      send_test();
      return;
  }
});

export {};
