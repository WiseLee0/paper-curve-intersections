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
      <svg width="107" height="103" viewBox="0 0 107 103" fill="none" xmlns="http://www.w3.org/2000/svg" id="path1">
        <path d="M 0 0 L 304 188 M 0 0 L 76 0 M 0 0 L 0 47 M 0 0 L 304 0 M 0 0 L 0 188 M 0 0 L 0 94 M 0 0 L 152 0 M 0 0 L 304 94 M 0 0 L 152 188 M 0 0 L 228 0 M 0 0 L 304 47 M 0 0 L 304 141 M 0 0 L 76 188 M 0 0 L 228 188 M 0 0 L 0 141 M 304 0 L 0 188 M 304 0 L 152 188 M 304 0 L 228 0 M 304 0 L 304 47 M 304 0 L 304 188 M 304 0 L 0 94 M 304 0 L 152 0 M 304 0 L 304 94 M 304 0 L 76 0 M 304 0 L 304 141 M 304 0 L 76 188 M 304 0 L 228 188 M 304 0 L 0 47 M 304 0 L 0 141 M 304 188 L 304 141 M 304 188 L 228 188 M 304 188 L 0 188 M 304 188 L 0 94 M 304 188 L 152 0 M 304 188 L 304 94 M 304 188 L 152 188 M 304 188 L 76 0 M 304 188 L 228 0 M 304 188 L 304 47 M 304 188 L 76 188 M 304 188 L 0 47 M 304 188 L 0 141 M 0 188 L 76 188 M 0 188 L 0 141 M 0 188 L 0 94 M 0 188 L 152 0 M 0 188 L 304 94 M 0 188 L 152 188 M 0 188 L 76 0 M 0 188 L 228 0 M 0 188 L 304 47 M 0 188 L 304 141 M 0 188 L 228 188 M 0 188 L 0 47 M 0 94 L 152 0 M 0 94 L 152 188 M 0 94 L 304 47 M 0 94 L 0 47 M 0 94 L 0 141 M 0 94 L 304 94 M 0 94 L 76 0 M 0 94 L 304 141 M 0 94 L 76 188 M 0 94 L 228 188 M 152 0 L 304 94 M 152 0 L 152 188 M 152 0 L 76 0 M 152 0 L 228 0 M 152 0 L 304 141 M 152 0 L 76 188 M 152 0 L 228 188 M 152 0 L 0 47 M 152 0 L 0 141 M 304 94 L 152 188 M 304 94 L 304 47 M 304 94 L 304 141 M 304 94 L 0 47 M 304 94 L 76 0 M 304 94 L 228 0 M 304 94 L 76 188 M 304 94 L 228 188 M 304 94 L 0 141 M 152 188 L 76 0 M 152 188 L 228 0 M 152 188 L 76 188 M 152 188 L 228 188 M 152 188 L 304 47 M 152 188 L 304 141 M 152 188 L 0 47 M 152 188 L 0 141 M 76 0 L 0 47 M 76 0 L 228 0 M 76 0 L 304 47 M 76 0 L 304 141 M 76 0 L 76 188 M 76 0 L 228 188 M 76 0 L 0 141 M 228 0 L 304 47 M 228 0 L 304 141 M 228 0 L 76 188 M 228 0 L 228 188 M 228 0 L 0 141 M 304 47 L 0 141 M 304 47 L 304 141 M 304 47 L 76 188 M 304 47 L 228 188 M 304 47 L 0 47 M 304 141 L 76 188 M 304 141 L 228 188 M 304 141 L 0 47 M 304 141 L 0 141 M 76 188 L 0 47 M 76 188 L 0 141 M 76 188 L 228 188 M 228 188 L 0 47 M 228 188 L 0 141 M 0 47 L 0 141 M 10 10 L 314 198 M 10 10 L 86 10 M 10 10 L 10 57 M 10 10 L 314 10 M 10 10 L 10 198 M 10 10 L 10 104 M 10 10 L 162 10 M 10 10 L 314 104 M 10 10 L 162 198 M 10 10 L 238 10 M 10 10 L 314 57 M 10 10 L 314 151 M 10 10 L 86 198 M 10 10 L 238 198 M 10 10 L 10 151 M 314 10 L 10 198 M 314 10 L 162 198 M 314 10 L 238 10 M 314 10 L 314 57 M 314 10 L 314 198 M 314 10 L 10 104 M 314 10 L 162 10 M 314 10 L 314 104 M 314 10 L 86 10 M 314 10 L 314 151 M 314 10 L 86 198 M 314 10 L 238 198 M 314 10 L 10 57 M 314 10 L 10 151 M 314 198 L 314 151 M 314 198 L 238 198 M 314 198 L 10 198 M 314 198 L 10 104 M 314 198 L 162 10 M 314 198 L 314 104 M 314 198 L 162 198 M 314 198 L 86 10 M 314 198 L 238 10 M 314 198 L 314 57 M 314 198 L 86 198 M 314 198 L 10 57 M 314 198 L 10 151 M 10 198 L 86 198 M 10 198 L 10 151 M 10 198 L 10 104 M 10 198 L 162 10 M 10 198 L 314 104 M 10 198 L 162 198 M 10 198 L 86 10 M 10 198 L 238 10 M 10 198 L 314 57 M 10 198 L 314 151 M 10 198 L 238 198 M 10 198 L 10 57 M 10 104 L 162 10 M 10 104 L 162 198 M 10 104 L 314 57 M 10 104 L 10 57 M 10 104 L 10 151 M 10 104 L 314 104 M 10 104 L 86 10 M 10 104 L 314 151 M 10 104 L 86 198 M 10 104 L 238 198 M 162 10 L 314 104 M 162 10 L 162 198 M 162 10 L 86 10 M 162 10 L 238 10 M 162 10 L 314 151 M 162 10 L 86 198 M 162 10 L 238 198 M 162 10 L 10 57 M 162 10 L 10 151 M 314 104 L 162 198 M 314 104 L 314 57 M 314 104 L 314 151 M 314 104 L 10 57 M 314 104 L 86 10 M 314 104 L 238 10 M 314 104 L 86 198 M 314 104 L 238 198 M 314 104 L 10 151 M 162 198 L 86 10 M 162 198 L 238 10 M 162 198 L 86 198 M 162 198 L 238 198 M 162 198 L 314 57 M 162 198 L 314 151 M 162 198 L 10 57 M 162 198 L 10 151 M 86 10 L 10 57 M 86 10 L 238 10 M 86 10 L 314 57 M 86 10 L 314 151 M 86 10 L 86 198 M 86 10 L 238 198 M 86 10 L 10 151 M 238 10 L 314 57 M 238 10 L 314 151 M 238 10 L 86 198 M 238 10 L 238 198 M 238 10 L 10 151 M 314 57 L 10 151 M 314 57 L 314 151 M 314 57 L 86 198 M 314 57 L 238 198 M 314 57 L 10 57 M 314 151 L 86 198 M 314 151 L 238 198 M 314 151 L 10 57 M 314 151 L 10 151 M 86 198 L 10 57 M 86 198 L 10 151 M 86 198 L 238 198 M 238 198 L 10 57 M 238 198 L 10 151 M 10 57 L 10 151"
 fill="#D9D9D9" stroke="#FC1515" />
      </svg>
    </div>
  );
}
