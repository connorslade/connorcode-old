paypal
  .Buttons({
    style: {
      shape: "rect",
      color: "gold",
      layout: "vertical",
      label: "paypal",
    },
    createOrder: function (_data, actions) {
      return actions.order.create({
        purchase_units: [
          {
            amount: {
              value: "1.5",
            },
          },
        ],
      });
    },
    onApprove: function (_data, actions) {
      return actions.order.capture().then(function (details) {
        alert(
          "Transaction completed by " + details.payer.name.given_name + "!"
        );
        localStorage.setItem("payed", true);
        location.href = "payed.html";
      });
    },
  })
  .render("#paypal-button-container");
