const worker: Worker = self as unknown as Worker;

let hobot: { get_build: Function, get_test: Function } = await import('../hobot/pkg');

worker.postMessage({
  type: 'test_script',
  test: hobot.get_test(),
})

async function build(script: string) {
  try {
    const [tokens, graph] = hobot.get_build(script);
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

worker.addEventListener('message', (msg) => {
  switch (msg.data.type) {
    case 'build':
      build(msg.data.script);
      return;
  }
});

export {};
