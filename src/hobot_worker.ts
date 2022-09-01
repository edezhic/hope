const worker: Worker = self as unknown as Worker;

let hobot: { get_build: Function, get_test: Function } = await import('../hobot/pkg');

worker.addEventListener('message', ({ data: { type, script } }) => {
  if (type == 'build') {
    try {
      const [tokens, graph] = hobot.get_build(script);
      worker.postMessage({
        type: 'build',
        tokens,
        graph,
      });
    } catch (error) {
      worker.postMessage({
        type: 'error',
        error
      })
    }
  } else {
    worker.postMessage({
      type: 'error',
      error: 'Unknown request'
    })
  }
});

worker.postMessage({
  type: 'test_script',
  test: hobot.get_test(),
})

export {};
