const worker: Worker = self as unknown as Worker;

async function get_build_script(script: string) {
  try {
    const { get_build } = await import('../hobot/pkg');
    const [tokens, graph] = get_build(script);
    worker.postMessage({
      type: 'build',
      tokens,
      graph,
    });
  } catch (e) {
    worker.postMessage({
      type: 'error',
      e
    })
  }
}

async function send_test() {
  const { get_test } = await import('../hobot/pkg');
  worker.postMessage({
    type: 'test',
    test: get_test(),
  });
}

worker.addEventListener('message', (evt) => {
  switch (evt.data.type) {
    case 'get_build':
      get_build_script(evt.data.script);
      return;
    case 'get_test':
      send_test();
      return;
  }
});

export {};
