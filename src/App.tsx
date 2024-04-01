import { useEffect } from "react";
import { getIntersections } from './intersections'
import paper from 'paper'
import initwasm, { rust_get_intersections } from '../intersections/pkg'
const pathToVectorNetwork = (path: any) => {
  const curves = path.getCurves()
  const res = []
  for (let i = 0; i < curves.length; i++) {
    const curve = curves[i].getValues();
    res.push(curve)
  }
  return res
}
export default function App() {
  useEffect(() => {
    main()
  }, [])

  const main = async () => {
    await initwasm()
    paper.setup('canvas-id');
    const words1 = paper.project.importSVG(document.getElementById('path1') as any).children[1] as any;
    const inrsections1 = pathToVectorNetwork(words1)

    console.time('wasm time')
    const wasm_res = rust_get_intersections(inrsections1, inrsections1, true);
    console.timeEnd('wasm time')
    console.log(wasm_res);

    const myIntersections: any = []
    console.time('time')
    getIntersections(inrsections1, undefined, myIntersections)
    console.timeEnd('time')
    const set1 = new Set()
    for (let i = 0; i < myIntersections.length; i++) {
      const item = myIntersections[i];
      set1.add(`${item[1].toFixed(2)}#${item[2].toFixed(2)}`)
    }
    console.log(myIntersections);
    console.log(set1);


    const paperIntersections = []
    console.time('time2')
    const intersections = words1.getIntersections()
    for (let j = 0; j < intersections.length; j++) {
      const item = intersections[j];
      const temp = []
      temp.push(item.time)
      temp.push(item.point.x)
      temp.push(item.point.y)
      temp.push(item.intersection.time)
      temp.push(item.intersection.point.x)
      temp.push(item.intersection.point.y)
      paperIntersections.push([...temp])
    }
    console.timeEnd('time2')
    const set2 = new Set()
    for (let i = 0; i < paperIntersections.length; i++) {
      const item = paperIntersections[i];
      set2.add(`${item[1].toFixed(2)}#${item[2].toFixed(2)}`)
    }
    console.log(paperIntersections);
    console.log(set2);
    for (const v of set2) {
      if (!set1.has(v)) {
        console.log(v);
      }
    }
  }

  return (
    <div>
      <canvas id="canvas-id"></canvas>
      <svg width="100" height="100" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg" id="path1">
        <path d="M 99.99999237060547 50 C 99.99999237060547 22.38576316833496 22.38576316833496 100 50 100 M 99.99999237060547 50 C 99.99999237060547 22.38576316833496 0 22.38576316833496 0 50 M 99.99999237060547 50 C 99.99999237060547 77.61423683166504 22.38576316833496 0 50 0 M 50 100 C 77.61423683166504 100 0 22.38576316833496 0 50 M 50 100 C 22.38576316833496 100 77.61423683166504 0 50 0 M 0 50 C 0 77.61423683166504 77.61423683166504 0 50 0 M 14.500004768371582 17.5 C 40.63394260406494 48.17949104309082 59.46272277832031 63.936296463012695 97.49999237060547 90.5 M 85.49999237060547 0 C 39.038124084472656 34.40242004394531 30.820515155792236 57.41726303100586 23.499998092651367 100 M 70.49998474121094 100 C 92.18121337890625 55.2330322265625 96.43761444091797 31.7164306640625 32.99998474121094 4 M 109.99999237060547 60 C 109.99999237060547 32.38576316833496 32.38576316833496 110 60 110 M 109.99999237060547 60 C 109.99999237060547 32.38576316833496 10 32.38576316833496 10 60 M 109.99999237060547 60 C 109.99999237060547 87.61423683166504 32.38576316833496 10 60 10 M 60 110 C 87.61423683166504 110 10 32.38576316833496 10 60 M 60 110 C 32.38576316833496 110 87.61423683166504 10 60 10 M 10 60 C 10 87.61423683166504 87.61423683166504 10 60 10 M 24.500003814697266 27.5 C 50.633941650390625 58.17949104309082 69.46272277832031 73.9362964630127 107.49999237060547 100.5 M 95.49999237060547 10 C 49.038124084472656 44.40242004394531 40.82051706314087 67.41726303100586 33.5 110 M 80.49998474121094 110 C 102.18121337890625 65.2330322265625 106.43761444091797 41.7164306640625 42.99998474121094 14 M 119.99999237060547 70 C 119.99999237060547 42.38576316833496 42.38576316833496 120 70 120 M 119.99999237060547 70 C 119.99999237060547 42.38576316833496 20 42.38576316833496 20 70 M 119.99999237060547 70 C 119.99999237060547 97.61423683166504 42.38576316833496 20 70 20 M 70 120 C 97.61423683166504 120 20 42.38576316833496 20 70 M 70 120 C 42.38576316833496 120 97.61423683166504 20 70 20 M 20 70 C 20 97.61423683166504 97.61423683166504 20 70 20 M 34.500003814697266 37.5 C 60.633941650390625 68.17949104309082 79.46272277832031 83.9362964630127 117.49999237060547 110.5 M 105.49999237060547 20 C 59.038124084472656 54.40242004394531 50.82051706314087 77.41726303100586 43.5 120 M 90.49998474121094 120 C 112.18121337890625 75.2330322265625 116.43761444091797 51.7164306640625 52.99998474121094 24 M 130 80 C 130 52.38576316833496 52.38576316833496 130 80 130 M 130 80 C 130 52.38576316833496 30 52.38576316833496 30 80 M 130 80 C 130 107.61423683166504 52.38576316833496 30 80 30 M 80 130 C 107.61423683166504 130 30 52.38576316833496 30 80 M 80 130 C 52.38576316833496 130 107.61423683166504 30 80 30 M 30 80 C 30 107.61423683166504 107.61423683166504 30 80 30 M 44.500003814697266 47.5 C 70.63394165039062 78.17949104309082 89.46272277832031 93.9362964630127 127.49999237060547 120.5 M 115.49999237060547 30 C 69.03812408447266 64.40242004394531 60.82051706314087 87.41726303100586 53.5 130 M 100.49998474121094 130 C 122.18121337890625 85.2330322265625 126.43761444091797 61.7164306640625 62.99998474121094 34 M 140 90 C 140 62.38576316833496 62.38576316833496 140 90 140 M 140 90 C 140 62.38576316833496 40 62.38576316833496 40 90 M 140 90 C 140 117.61423683166504 62.38576316833496 40 90 40 M 90 140 C 117.61423683166504 140 40 62.38576316833496 40 90 M 90 140 C 62.38576316833496 140 117.61423683166504 40 90 40 M 40 90 C 40 117.61423683166504 117.61423683166504 40 90 40 M 54.500003814697266 57.5 C 80.63394165039062 88.17949104309082 99.46273040771484 103.9362964630127 137.5 130.5 M 125.49999237060547 40 C 79.03812408447266 74.40242004394531 70.82051706314087 97.41726303100586 63.5 140 M 110.49998474121094 140 C 132.18121337890625 95.2330322265625 136.43761444091797 71.7164306640625 72.99998474121094 44"
          stroke="#D9D9D9" />
      </svg>
    </div>
  );
}
