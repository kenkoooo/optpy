__v0 = 1000000000000000
__v1 = map_int(input().split())
__v2 = __v1[0]
__v3 = __v1[1]
__v4 = list(map_int(input().split()))
__v5 = ((__v2 * 2) + 2)
__v6 = (__v2 * 2)
__v7 = ((__v2 * 2) + 1)
def __f0(__v8):
    __v9 = []
    __v10 = list(range__macro__(__v8))
    __v10.reverse()
    while (len(__v10) > 0):
        __v11 = __v10.pop()
        __v9.append([])
    
    return __v9

__v12 = __f0(__v5)
def __f1(__v13, __v14, __v15, __v16, __v17):
    __v17[__v13].append([__v14, __v15, __v16, len(__v17[__v14])])
    __v17[__v14].append([__v13, 0, (-__v16), (len(__v17[__v13]) - 1)])

def __f2(__v18, __v19, __v20, __v21):
    __v22 = ([__v19] * __v21)
    __v22[__v18] = 0
    __v23 = ([0] * __v21)
    __v24 = ([0] * __v21)
    while True:
        __v25 = False
        __v26 = list(range__macro__(__v21))
        __v26.reverse()
        while (len(__v26) > 0):
            __v27 = __v26.pop()
            if (__v22[__v27] == __v19):
                continue
            
            else:
                pass
            
            __v28 = list(range__macro__(len(__v20[__v27])))
            __v28.reverse()
            while (len(__v28) > 0):
                __v29 = __v28.pop()
                __v30 = __v20[__v27][__v29]
                __v31 = __v30[0]
                __v32 = __v30[1]
                __v33 = __v30[2]
                __v34 = __v30[3]
                if ((__v32 > 0)) and ((__v22[__v31] > (__v22[__v27] + __v33))):
                    __v22[__v31] = (__v22[__v27] + __v33)
                    __v25 = True
                    __v23[__v31] = __v27
                    __v24[__v31] = __v29
                
                else:
                    pass
                
            
        
        if (not __v25):
            break
        
        else:
            pass
        
    
    return (__v22, __v23, __v24)

def __f3(__v35, __v36, __v37, __v38, __v39, __v40):
    __v41 = 0
    while (__v37 > 0):
        __v42 = __f2(__v35, __v38, __v39, __v40)
        __v43 = __v42[0]
        __v44 = __v42[1]
        __v45 = __v42[2]
        if (__v43[__v36] == __v38):
            return __v38
        
        else:
            pass
        
        __v46 = __v37
        __v47 = __v36
        while (__v47 != __v35):
            __v46 = min__macro__(__v46, __v39[__v44[__v47]][__v45[__v47]][1])
            __v47 = __v44[__v47]
        
        __v41 = (__v41 + (__v46 * __v43[__v36]))
        __v37 = (__v37 - __v46)
        __v47 = __v36
        while (__v47 != __v35):
            __v39[__v44[__v47]][__v45[__v47]][1] = (__v39[__v44[__v47]][__v45[__v47]][1] - __v46)
            __v48 = __v39[__v44[__v47]][__v45[__v47]][3]
            __v39[__v47][__v48][1] = (__v39[__v47][__v48][1] + __v46)
            __v47 = __v44[__v47]
        
    
    return __v41

__v49 = list(range__macro__(__v2))
__v49.reverse()
while (len(__v49) > 0):
    __v50 = __v49.pop()
    __f1(__v6, __v50, 1, 0, __v12)
    __f1(__v50, __v7, 1, __v3, __v12)

__v51 = list(range__macro__(__v2))
__v51.reverse()
while (len(__v51) > 0):
    __v50 = __v51.pop()
    __v52 = list(range__macro__((__v50 + 1), __v2))
    __v52.reverse()
    while (len(__v52) > 0):
        __v53 = __v52.pop()
        __f1(__v50, (__v2 + __v53), 1, abs((__v4[__v50] - __v4[__v53])), __v12)
    

__v54 = list(range__macro__(__v2))
__v54.reverse()
while (len(__v54) > 0):
    __v53 = __v54.pop()
    __f1((__v2 + __v53), __v7, 1, 0, __v12)

__v55 = __f3(__v6, __v7, __v2, __v0, __v12, __v5)
print_values__macro__(__v55)
