import React from 'react';
import { HiOutlineInformationCircle } from "react-icons/hi";

import Productdef from "./Productdef";
import {CompareCheck}  from "./Compare";
import './Container.css';


const Specifications = ({specifications}) => {
  let specs = specifications;
  try {
    specs = JSON.parse(specifications);
  }
  catch(err) { return (<p className="unavail"> Unavailable </p>) }
  return (
    <table className="specTable">
      {
        Object.keys(specs).map(key => {
          const value = specs[key];
          return (
            <tr className='spec'>
              <td>
                {key}
              </td>
              <td style={{wordWrap:'normal'}}>
                {value}
              </td>
            </tr>
          )
        })
      }
    </table>
  )
}


const Container = ({prod,CSet,setCSet}) => {

  //let name = product.name;
  if (prod.price === "Not Available") { 
    return null;
  }

  //if(name.length > 110) {
    //name = `${name.slice(0,110)}...`
  //}

  return (
    <>    
          <div className="product-container">
              <a href={prod.url} target="_blank" rel="noreferrer">
                <div className="product-image">
                    <img src={prod.img} alt="error"/>
                </div>
                <Productdef
                  name={prod.name}
                  price={prod.price}
                  store={"./listing/" + prod.store + ".png"}
                  url={prod.url}
                  warranty={prod.warranty}
                  returnPolicy={prod.return_replace}
                  //availibility="Availibility"
                  rating={Math.floor(Math.random() * (2) ) + 3}
                />
              </a>
            <div className="product-opts">
              <div class="specs-section">
                <h5 class="specs-icon"><HiOutlineInformationCircle /></h5>
                <div class="specs-float-section">
                  <h3 className="spec-float-title">Specifications</h3>
                    <Specifications specifications={(prod.specs)}/>
                </div>
              </div>
              <CompareCheck pid={prod.id} CSet={CSet} setCSet={setCSet}/>
            </div>
        </div>
        </>
  )
}


export default Container
