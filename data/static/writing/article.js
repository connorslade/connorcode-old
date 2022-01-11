// function data() {
//   return {
//     path: "{{PATH}}",
//     genPath: () => {
//       return `writing <i class="fa fa-angle-right"></i> ${this.path}`;
//     },
//   };
// }


function loadPath(path) {
  let out = `<a href="/writing">writing</a> `;

  path.split('/').forEach((item) => {
    out += ` <i class='fa fa-angle-right'></i> <a href="/writing/${item}">${item}</a>`;
  });

  return out;
}
