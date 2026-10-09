import * as http from "node:http";
import { json } from "node:stream/consumers";
import * as util from "node:util";

const server = http
	.createServer(async (req, res) => {
		const { pathname, searchParams } = new URL(`http://t${req.url}`);
		const { method, headersDistinct } = req;
		const body = await json(req).catch(() => null);
		console.log(
			util.inspect(
				{ pathname, searchParams, method, body, headersDistinct },
				false,
				10,
				true,
			),
		);
		res.setHeaders(
			new Map([
				["Access-Control-Allow-Origin", "*"],
				["Access-Control-Allow-Methods", "POST"],
				["Access-Control-Allow-Headers", "Content-Type"],
			]),
		);
		res.statusCode = 204;
		res.end();
	})
	.listen(4174);

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
