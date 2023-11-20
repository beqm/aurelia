"use strict";

const args = process.argv.slice(2);
const VISION_KEY = args[0];
const VISION_ENDPOINT = args[1];

import request from "request";

let key = VISION_KEY;
let endpoint = VISION_ENDPOINT;

var uriBase = endpoint + "vision/v3.1/analyze";

const imageUrl = "D:/Vault/Coding/Repositories/aurelia/data/display.png";

// Request parameters.
const params = {
  visualFeatures: "Categories,Description,Color",
  details: "",
  language: "en",
};

const options = {
  uri: uriBase,
  qs: params,
  body: '{"url": ' + '"' + imageUrl + '"}',
  headers: {
    "Content-Type": "application/octet-stream",
    "Ocp-Apim-Subscription-Key": key,
  },
};

console.log(options);

request.post(options, (error, response, body) => {
  console.log(response);
  if (error) {
    console.log("Error: ", error);
    return;
  }
  let jsonResponse = JSON.stringify(JSON.parse(body), null, "  ");
  console.log("JSON Response\n");
  console.log(jsonResponse);
});
