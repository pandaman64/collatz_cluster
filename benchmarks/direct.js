import http from "k6/http";
import { randomSeed } from "k6";
import { Counter } from "k6/metrics";

const cache_hit_metrics = new Counter("cache_hit");

export let options = {
    vus: "30",
    duration: "30s",
};

export default function() {
    randomSeed(__VU * 31 + __ITER);
    let number = Math.floor(Math.random() * 10000);
    let response = http.get(`http://localhost:8000/steps/${number}`);
    let cache_hit = response.json()["cache_hit"];
    if (cache_hit) {
        cache_hit_metrics.add(1)
    }
};
