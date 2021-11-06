import "../style/style.scss";
window.onload = function () {
    gapi.load("client:auth2", function () {
        import("../pkg");
    });
}
