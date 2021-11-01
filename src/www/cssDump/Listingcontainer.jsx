import React from 'react';
//import './Listingcontainer.css';
import { BsFillInfoCircleFill } from "react-icons/bs";
import Productdef from "./Productdef";

function Listingcontainer() {
  return (
    <>
      <div className="product_container">

        <div className="leftdiv">
          <div className="prod_img">
            {/* <img src="/toys-category.jpg" alt="error" /> */}
          </div>

          <Productdef
            name="Product name"
            price="Price"
            availibility="Availibility"
            rating="Rating"

          />

          <div class="popover__wrapper">
            
              <h2 class="popover__title"><BsFillInfoCircleFill /></h2>
            
            <div class="popover__content">
              <p class="popover__message">Specifications</p>
            </div>
          </div>
        </div>


        <div className="rightdiv">
          <div className="prod_img">
            {/* <img src="/toys-category.jpg" alt="error" /> */}
          </div>

          <Productdef
            name="Product name"
            price="price"
            availibility="Availibility"
            rating="Rating"
          />

          <div class="popover__wrapper">
          
              <h2 class="popover__title"><BsFillInfoCircleFill /></h2>
            
            <div class="popover__content">
              <p class="popover__message">Specifictions</p>
            </div>
          </div>
        </div>

      </div>
    </>
  )
}

export default Listingcontainer;
