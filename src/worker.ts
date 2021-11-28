const ctx: Worker = self as unknown as Worker;

async function translate(script: string) {
  const { tokenize } = await import('../core/pkg');
  //const test_script = 'X is 0.5, y is 1.5. Add x to y, show result.';
  ctx.postMessage({
    type: 'translation',
    //script: test_script,
    tokens: tokenize(script),
  });
}

ctx.addEventListener('message', (evt) => {
  switch (evt.data.type) {
    case 'translate':
      translate(evt.data.script);
      return;
  }
});

export {};
