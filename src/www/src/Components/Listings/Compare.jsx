import React, { useState, useEffect, useRef, useCallback } from "react";

import { ReactComponent as Scale } from './assets/cmp/compare.svg';

import './Compare.css';

const CompareCheck = ({pid,CSet,setCSet}) => {
    const [isCmpActive, setCmpActive] = useState(false);
    const onCbtnClick = () => setCmpActive(!isCmpActive);
    useEffect(() => {
        if (isCmpActive) {
            setCSet(c => new Set([...c, pid]));
            //setCSet(c => [...c, pid]);
        }
        else { 
            setCSet(c => new Set([...c].filter(item => item!= pid)));
            //setCSet(c =>[...c].filter(item => item!= pid));
        }
    },[isCmpActive]);
    return (
        <>
            <div className={`compare-button ${isCmpActive ? 'check': ''}`} onClick={onCbtnClick}>
              <Scale/>
            </div>
        </>
    )
};

const CompareTable = ({cProducts}) => {
    const [specList, setSpecList ] = useState([]);
    const [sKeys, setSKeys ] = useState(new Set());

    const filterSpecs = (pList) => {
        let _set = new Set();
        let _arr = [];
        pList.map(lItem => {
            let parsedSpec = JSON.parse(lItem.specs);
            _arr.push(parsedSpec);
            Object.keys(parsedSpec).forEach(item => _set.add(item.trim())); 
        });
        return [_set, _arr];
    };
    

    useEffect(() => {
        let [uniqKeys, slist] = filterSpecs(cProducts);
        setSKeys(new Set(Array.from(uniqKeys).sort()));
        setSpecList(slist)
        //console.log(sKeys,specList);
    },[cProducts]);


    return (
        <>
            <table className="compare-table">
                <tr className="compare-row-image">
                    <th /> {/* Top Left Blank Gap*/}
                    {
                        cProducts.map(product => {
                            return (
                                <td>
                                    <a href={product.url} target="_blank" rel="noreferer">
                                    <img className="cProdImg" src={product.img} alt="error"/>
                                    <img className="storeIC" src={'/listing/' + product.store + '.svg'}/>
                                    </a>
                                </td>
                        )})
                    }
                </tr>
                <tr className="row_pname">
                    <th className="cTH">Product:</th>
                    {
                        cProducts.map(product => { return( <td> {product.name} </td>)})
                    }
                </tr>
                {
                    Array.from(sKeys).sort().map(key => {
                        return(
                            <tr>
                                <th className="cTH">{key}</th>
                                {
                                    specList.map(specObj => {
                                        let sVal = specObj[key];
                                        return(<td className="specCells">{(sVal != null) ? sVal : "-"}</td>)
                                    })
                                }
                            </tr>
                        )
                    })
                }
            </table>
        </>
    )
};

const Compare = ({products,CSet}) => {
    let [len, setLen] = useState(CSet.size);
    //let [len, setLen] = useState(CSet.length);
    const filterProducts = (c1,c2) => { return c1.filter(elem => c2.has(elem.id))}
    const [prodData, setProdData] = useState([]); 

    const [isCWinActive, toggleCWin] = useState(false);
    const onCBarClick = () => toggleCWin(!isCWinActive);

    useEffect(()=> {
        //setProdData(products.filter(product => CSet.has(product.id)));
        setLen(CSet.size);
        setProdData(filterProducts(products,CSet));
        //console.log(CSet,prodData);
    },[CSet]);

    return (
        <>
            <div onClick={onCBarClick} className={`compare-bar ${len > 1 ? 'active': 'inactive'}`}>
                <h6>{len} items to compare..</h6>
            </div>
            <div className={`compare-window ${isCWinActive ? 'active': ''}`}>
                <CompareTable cProducts={prodData} />
            </div>
        </>
    )
};

export { Compare, CompareCheck };
