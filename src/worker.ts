const ctx: Worker = self as unknown as Worker;

async function translate(script: string) {
  const { tokenize } = await import('../core/pkg');
  ctx.postMessage({
    type: 'translation',
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
