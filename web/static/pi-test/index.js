const SIZE = 20;

// https://stackoverflow.com/questions/30747235/javascript-pi-%CF%80-calculator
function* piIter() {
  let q = 1n;
  let r = 180n;
  let t = 60n;
  let i = 2n;
  while (true) {
    let digit = ((i * 27n - 12n) * q + r * 5n) / (t * 5n);
    yield Number(digit);
    let u = i * 3n;
    u = (u + 1n) * 3n * (u + 2n);
    r = u * 10n * (q * (i * 5n - 2n) + r - t * digit);
    q *= 10n * i * (i++ * 2n - 1n);
    t *= u;
  }
}

let iter = piIter();

function initPi() {
  let out = [];
  iter.next();
  for (let i = 0; i < SIZE; i++) out.push(iter.next().value);
  return out;
}

function init() {
  let highScore = localStorage.getItem("pi-test:high-score") ?? 0;
  document.querySelector("[high-score]").innerText = highScore;

  return {
    pi: initPi(),
    index: -1,
    running: true,
    highScore,

    checkValidInc: (e, index, pi) => {
      let res = checkResult(e.key, pi[index]);
      pi.push(iter.next().value);
      return { res, digit: pi[index] };
    },
  };
}

function checkResult(num, digit) {
  if (isNaN(num)) return 0;
  if (num != digit) return 1;
  return 2;
}
