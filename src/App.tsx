import { useEffect } from "react";
import { getIntersections } from './intersections'
import paper, { CompoundPath } from 'paper'
import { rust_get_intersections } from '../intersections/pkg'
const pathToVectorNetwork = (paths: any) => {
  const res = []
  for (let i = 0; i < paths.length; i++) {
    const path = paths[i];
    const curves = path.getCurves()
    for (let j = 0; j < curves.length; j++) {
      const curve = curves[j].getValues();
      res.push(curve)
    }
  }
  return res
}

const svgPath = "M 44.55659972808161 105.84395575676466 C 42.93455116632601 103.22671522565699 41.40157636398038 100.39968005391214 39.95847903189694 97.43873532912347 M 107.5 37.8 C 104.62660435071061 72.49358507624493 10 111.56 10 111.56"

export default function App() {
  useEffect(() => {
    main()
  }, [])

  const main = async () => {
    paper.setup('canvas-id');

    const words1 = new CompoundPath(svgPath)
    const inrsections1 = pathToVectorNetwork(words1.children)
    console.log(inrsections1);
    
    console.time('wasm')
    const wasm_res = rust_get_intersections(new Float64Array(inrsections1.flat()));
    const wasm_chunk = chunkArray(wasm_res, 8)
    console.timeEnd('wasm')
    const set1 = new Set()
    for (let i = 0; i < wasm_chunk.length; i++) {
      const item = wasm_chunk[i];
      set1.add(`${item[0].toFixed(8)}#${item[2].toFixed(8)}#${item[3].toFixed(8)}`)
    }
    console.log(wasm_chunk);

    const myIntersections: any = []
    getIntersections(inrsections1, undefined, myIntersections)
    console.log(myIntersections);

    const paperIntersections = []
    console.time('paperjs')
    const intersections = (words1 as any).getIntersections().filter((item: any) => !item._overlap && item.isCrossing())
    console.timeEnd('paperjs')
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
    const set2 = new Set()
    for (let i = 0; i < paperIntersections.length; i++) {
      const item = paperIntersections[i];
      set2.add(`${item[0].toFixed(8)}#${item[1].toFixed(8)}#${item[2].toFixed(8)}`)
    }
    console.log(paperIntersections);

    for (const v of set2) {
      if (!set1.has(v)) {
        console.log(v);
        break;
      }
    }
  }

  return (
    <div>
      <canvas id="canvas-id"></canvas>
      <svg width="300" height="300" viewBox="0 0 300 300" fill="none" xmlns="http://www.w3.org/2000/svg" id="path1">
        <path d={svgPath} stroke="#D9D9D9" />
      </svg>
    </div>
  );
}


function chunkArray(array: Float64Array, chunkSize: number) {
  let result = [];
  for (let i = 0; i < array.length; i += chunkSize) {
    let chunk = array.slice(i, i + chunkSize);
    result.push(chunk);
  }

  return result;
}