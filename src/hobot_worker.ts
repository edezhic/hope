const core: Worker = self as unknown as Worker;

async function build_script(title: string, body: string) {
  const { build } = await import('../hobot/pkg');
  core.postMessage({
    type: 'tokens',
    tokens: build(title, body),
  });
}

async function send_tests() {
  const { get_tests } = await import('../hobot/pkg');
  core.postMessage({
    type: 'tests',
    tests: get_tests(),
  });
}

core.addEventListener('message', (evt) => {
  switch (evt.data.type) {
    case 'build':
      build_script(evt.data.title, evt.data.body);
      return;
    case 'get_tests':
      send_tests();
      return;
  }
});

export {};
