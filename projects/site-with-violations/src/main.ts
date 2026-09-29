import "./style.css";

new ReportingObserver(
	(reports, observer) => {
		console.log("report observed", reports, observer);
	},
	{ buffered: true },
).observe();

window.addEventListener("securitypolicyviolation", (event) => {
	console.log("report evented", event);
});

const dataImage = new Image(1, 1);
dataImage.src =
	"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAD0lEQVR42gEEAPv/AAAAvADAAL0PT/jcAAAAAElFTkSuQmCC";
dataImage.alt = "blue pixel";
document.body.appendChild(dataImage);

const blobImage = new Image(1, 1);
blobImage.src = URL.createObjectURL(new Blob([]));
blobImage.alt = "broken pixel";

document.body.appendChild(blobImage);
