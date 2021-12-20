//   __  __      __     __  ___ 
//  /  `/  \|\/||__)/\ |__)|__  
//  \__,\__/|  ||  /~~\|  \|___ 
//                              

// Library Imports
import React, { useState, useEffect } from "react";

// File Imports
import { ReactComponent as Scale } from './assets/compare.svg';
import Product from 'Components/Listings/cogs/product';

// CSS
import './compare.css';

export interface cmpActions {
    do: 'add' | 'delete';
    pid: string;
}
function cmpReducer(state: Set<string|unknown> , action: cmpActions): Set<string|unknown> {
    switch (action.do) {
        case 'add': 
            return state.add(action.pid);
        case 'delete': 
            return state.delete(action.pid) && state || state;
        default:
            return state;
    }
}

/** `Add to Compare Button` */
const Checkbox = ({pid,setCompare}: {pid: string, setCompare: React.Dispatch<cmpActions>}) => {
    const [isCmpActive, setCmpActive] = useState<boolean>(false);
    const onCbtnClick = (e: React.MouseEvent<HTMLButtonElement>) => {
        e.stopPropagation();
        setCmpActive(!isCmpActive);
    };
    useEffect(() => {
        (isCmpActive) 
                ? setCompare({do: 'add'   , pid: pid})
                : setCompare({do: 'delete', pid: pid})
            //setCompare(c => new Set([...c, pid]));
        //}
        //else { 
            //setCompare(c => new Set([...c].filter(item => item!= pid)));
        //}
    },[isCmpActive]);
    return (
            <button className={`compare-button ${isCmpActive ? 'check': ''}`} onClick={onCbtnClick}>
              <Scale />
            </button>
    )
};

/** `Product Comparison Table` */
const Table = ({cProducts}:{cProducts: Product[]}) => {
  const [specList, setSpecList ] = useState<object[]>([]);
  const [sKeys, setSKeys ] = useState<Set<string>>(new Set());

    const filterSpecs = (pList: Product[]): [Set<string>,object[]] => {
      let _set: Set<string> = new Set(); // Set of Specification Keys
        let _arr: object[] = []; // Array of all the specs objects
        pList.forEach(lItem => {
            let spec = (lItem.specs) ? lItem.specs : {};
            _arr.push(spec);
            Object.keys(spec).forEach(item => _set.add(item.trim())); 
        });
        return [_set, _arr];
    };

    useEffect(() => {
        let [uniqKeys, slist] = filterSpecs(cProducts);
        setSKeys(new Set(Array.from(uniqKeys).sort()));
        setSpecList(slist);
        //console.log(sKeys,specList);
    },[cProducts]);

    return (
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
                                        let sVal = specObj[key as keyof typeof specObj];
                                        return(<td className="spec-cell">{(sVal != null) ? sVal : "-"}</td>)
                                    })
                                }
                            </tr>
                        )
                    })
                }
            </table>
    )
};


const Compare = ({products,compareSet}: {products: Product[], compareSet: Set<string|unknown>}) => {
    let [len, setLen] = useState(compareSet.size);
    //let [len, setLen] = useState(compareSet.length);
    const filterProducts = (c1: Product[],c2: Set<string|unknown>) => { return c1.filter(elem => c2.has(elem.id))}
    const [prodData, setProdData] = useState<Product[]>([]); 

    const [isCWinActive, toggleCWin] = useState(false);
    const onCBarClick = () => toggleCWin(!isCWinActive);

    useEffect(()=> {
        //setProdData(products.filter(product => compareSet.has(product.id)));
        setLen(compareSet.size);
        setProdData(filterProducts(products,compareSet));
        //console.log(compareSet,prodData);
    },[compareSet]);

    return (
        <>
            <div onClick={onCBarClick} className={`compare-bar ${len > 1 ? 'active': 'inactive'}`}>
                <h6>{len} items to compare..</h6>
            </div>
            <div className={`compare-window ${isCWinActive ? 'active': ''}`}>
                <Table cProducts={prodData} />
            </div>
        </>
    )
};

export { Checkbox, Table, cmpReducer };
export default Compare;
