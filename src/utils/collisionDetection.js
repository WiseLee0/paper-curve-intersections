export const CollisionDetection = {
    findCurveBoundsCollisions: function (curves1, curves2, is_self, tolerance) {
        function getBounds(curves) {
            var min = Math.min,
                max = Math.max,
                bounds = new Array(curves.length);
            for (var i = 0; i < curves.length; i++) {
                var v = curves[i];
                bounds[i] = [
                    min(v[0], v[2], v[4], v[6]),
                    min(v[1], v[3], v[5], v[7]),
                    max(v[0], v[2], v[4], v[6]),
                    max(v[1], v[3], v[5], v[7])
                ];
            }
            return bounds;
        }

        const bounds1 = getBounds(curves1)
        const bounds2 = is_self ? bounds1 : getBounds(curves2);
        return this.findBoundsCollisions(bounds1, bounds2, is_self, tolerance || 0);
    },
    findBoundsCollisions: function (boundsA, boundsB, is_self, tolerance,
        sweepVertical, onlySweepAxisCollisions) {
        const allBounds = is_self ? boundsA : boundsA.concat(boundsB)
        const lengthA = boundsA.length
        const lengthAll = allBounds.length


        function binarySearch(indices, coord, value) {
            var lo = 0,
                hi = indices.length;
            while (lo < hi) {
                var mid = (hi + lo) >>> 1;
                if (allBounds[indices[mid]][coord] < value) {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            return lo - 1;
        }
        var pri0 = sweepVertical ? 1 : 0,
            pri1 = pri0 + 2,
            sec0 = sweepVertical ? 0 : 1,
            sec1 = sec0 + 2;
        var allIndicesByPri0 = new Array(lengthAll);
        for (var i = 0; i < lengthAll; i++) {
            allIndicesByPri0[i] = i;
        }
        allIndicesByPri0.sort(function (i1, i2) {
            return allBounds[i1][pri0] - allBounds[i2][pri0];
        });
        var activeIndicesByPri1 = [],
            allCollisions = new Array(lengthA);
        for (var i = 0; i < lengthAll; i++) {
            var curIndex = allIndicesByPri0[i],
                curBounds = allBounds[curIndex],
                origIndex = is_self ? curIndex : curIndex - lengthA,
                isCurrentA = curIndex < lengthA,
                isCurrentB = is_self || !isCurrentA,
                curCollisions = isCurrentA ? [] : null;
            if (activeIndicesByPri1.length) {
                var pruneCount = binarySearch(activeIndicesByPri1, pri1,
                    curBounds[pri0] - tolerance) + 1;
                activeIndicesByPri1.splice(0, pruneCount);
                if (is_self && onlySweepAxisCollisions) {
                    curCollisions = curCollisions.concat(activeIndicesByPri1);
                    for (var j = 0; j < activeIndicesByPri1.length; j++) {
                        var activeIndex = activeIndicesByPri1[j];
                        allCollisions[activeIndex].push(origIndex);
                    }
                } else {
                    var curSec1 = curBounds[sec1],
                        curSec0 = curBounds[sec0];
                    for (var j = 0; j < activeIndicesByPri1.length; j++) {
                        var activeIndex = activeIndicesByPri1[j],
                            activeBounds = allBounds[activeIndex],
                            isActiveA = activeIndex < lengthA,
                            isActiveB = is_self || activeIndex >= lengthA;
                        if (
                            onlySweepAxisCollisions ||
                            (
                                isCurrentA && isActiveB ||
                                isCurrentB && isActiveA
                            ) && (
                                curSec1 >= activeBounds[sec0] - tolerance &&
                                curSec0 <= activeBounds[sec1] + tolerance
                            )
                        ) {
                            if (isCurrentA && isActiveB) {
                                curCollisions.push(
                                    is_self ? activeIndex : activeIndex - lengthA);
                            }
                            if (isCurrentB && isActiveA) {
                                allCollisions[activeIndex].push(origIndex);
                            }
                        }
                    }
                }
            }
            if (isCurrentA) {
                if (boundsA === boundsB) {
                    curCollisions.push(curIndex);
                }
                allCollisions[curIndex] = curCollisions;
            }
            if (activeIndicesByPri1.length) {
                var curPri1 = curBounds[pri1],
                    index = binarySearch(activeIndicesByPri1, pri1, curPri1);
                activeIndicesByPri1.splice(index + 1, 0, curIndex);
            } else {
                activeIndicesByPri1.push(curIndex);
            }
        }
        for (var i = 0; i < allCollisions.length; i++) {
            var collisions = allCollisions[i];
            if (collisions) {
                collisions.sort(function (i1, i2) { return i1 - i2; });
            }
        }
        return allCollisions;
    }
};