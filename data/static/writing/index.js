function data() {
  return {
    data: [],
    load() {
      fetch("/api/writing")
        .then((res) => res.json())
        .then((data) => {
          this.data = data.map((x) => {
            x.date = `<i class="fa fa-calendar"></i> ${x.date}`;
            return x;
          });
        });
    },
  };
}
