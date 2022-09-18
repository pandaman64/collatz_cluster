import http from "k6/http";
import { randomSeed } from "k6";

export let options = {
    vus: "30",
    duration: "30s",
};

export default function() {
    randomSeed(__VU);
    let number = Math.floor(Math.random() * 10000);
    let response = http.get(`http://localhost:8000/steps/${number}`);
};
