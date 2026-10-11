const dataImage = new Image(1, 1);
dataImage.src =
	"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAD0lEQVR42gEEAPv/AAAAvADAAL0PT/jcAAAAAElFTkSuQmCC";
dataImage.alt = "blue pixel";
document.body.appendChild(dataImage);

const blobImage = new Image(1, 1);
blobImage.src = URL.createObjectURL(new Blob([]));
blobImage.alt = "broken pixel";

document.body.appendChild(blobImage);

// Generate a permissions policy report
addEventListener("unload", (event) => {
	console.log("unload", event);
});
