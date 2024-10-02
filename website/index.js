// wait for the dom to load
document.addEventListener("DOMContentLoaded", function () {
  // get the form by id
  var form = document.getElementById("form");
  // add an event listener to the form
  form.addEventListener("submit", function (event) {
    // prevent the form from submitting
    event.preventDefault();
    // get the values of the form inputs
    var name = document.getElementById("name").value;
    var email = document.getElementById("email").value;
    var message = document.getElementById("message").value;
    // log the values to the console
    console.log(name, email, message);
  });
});

//get the id message and only allow 25 words, check this by counting 24 spaces
document.getElementById("message").addEventListener("input", function () {
  var message = this.value;
  var words = message.split(" ");
  if (words.length > 25) {
    this.value = words.slice(0, 25).join(" ");
  }
});
