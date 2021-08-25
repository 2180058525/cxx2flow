const rust = import('./pkg');

async function main() {
  const t = await rust
  const res = await t.generate(`
  int main() {
    int n = read(), m = read();
    for (int i = 1; i <= n; i++) f[i][0] = read();
  }
  `, 'main', true)
  console.log(res)
}

main();
  