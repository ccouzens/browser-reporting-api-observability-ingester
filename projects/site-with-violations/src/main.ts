import styleUrl from "./style.css?url";

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

const observerOutput = document.createElement("pre");
observerOutput.textContent = "Waiting for integrity-violation reports...";
document.body.appendChild(observerOutput);

if (typeof ReportingObserver === "undefined") {
	observerOutput.textContent = "ReportingObserver is not supported";
} else {
	const observed: Report[] = [];
	const observer = new ReportingObserver(
		(reports) => {
			for (const report of reports) {
				observed.push(report);
				console.log("integrity-violation", report);
			}
			observerOutput.textContent = JSON.stringify(observed, null, 2);
		},
		{ types: ["integrity-violation"], buffered: true },
	);
	observer.observe();
}

const styleLink = document.createElement("link");
styleLink.rel = "stylesheet";
styleLink.href = styleUrl;
document.head.appendChild(styleLink);
