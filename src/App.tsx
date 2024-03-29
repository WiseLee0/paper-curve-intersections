import { useEffect } from "react";
import { getIntersections } from './intersections'
import paper from 'paper'
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

  const main = () => {
    paper.setup('canvas-id');
    const words1 = paper.project.importSVG(document.getElementById('path1') as any).children[1] as any;
    const inrsections1 = pathToVectorNetwork(words1)

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
        <path d="M 106.13448333740234 52.76838684082031 C -115.86551666259766 -44.73158264160156 83.74869728088379 102.76840209960938 56.13446044921875 102.76840209960938 C 28.52022361755371 102.76840209960938 200.13448858261108 -17.231582641601562 6.1345343589782715 52.76838684082031 C 6.1345343589782715 25.154150009155273 187.63446044921875 15.768381834030151 56.13446044921875 2.76838755607605 C -75.36553955078125 -10.231606721878052 83.13448333740234 25.268417358398438 106.13448333740234 52.76838684082031 Z" fill="#D9D9D9" stroke="#FC1515" />
      </svg>
    </div>
  );
}
