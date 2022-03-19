const ctx: Worker = self as unknown as Worker;

async function build(title: string, body: string) {
  const { build } = await import('../core/pkg');
  ctx.postMessage({
    type: 'tokens',
    tokens: build(title, body),
  });
}

async function send_tests() {
  const { get_tests } = await import('../core/pkg');
  ctx.postMessage({
    type: 'tests',
    tests: get_tests(),
  });
}

ctx.addEventListener('message', (evt) => {
  switch (evt.data.type) {
    case 'build':
      build(evt.data.title, evt.data.body);
      return;
    case 'get_tests':
      send_tests();
      return;
  }
});

export {};
