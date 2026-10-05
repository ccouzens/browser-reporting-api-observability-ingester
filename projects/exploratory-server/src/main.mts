import * as http from "node:http";

const server = http.createServer(async (req, res) => {}).listen(4174);

server.addListener("listening", () => {
	const address = server.address();
	if (address === null) {
		console.log("server not running");
	} else {
		console.log(
			`server running at http://${typeof address === "string" ? address : `localhost:${address.port}`}/`,
			address,
		);
	}
});
