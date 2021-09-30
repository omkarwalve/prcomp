
import React, { useState } from 'react';
import './Navbar.css';
// import ListingNav from './ListingNav';
import { FaSearch } from "react-icons/fa";
import { FaShoppingCart } from "react-icons/fa";
import { FaUserCircle } from "react-icons/fa";


function Navbar() {
    // function cartpg(){
    //     <>
    //         < ListingNav />
    //     </>
    // }

            return (
            <div className="navBar">
                {/* <button onClick={(cartpg)}  >Click me</button> */}

                <button className="logo">logo</button>
                <select className="categoryDropdown">
                    <option selected value="all-cat">ALL</option>
                    <option value="cat-1">category1</option>
                    <option value="cat-2">category1</option>
                    <option value="cat-3">category1</option>
                    <option value="cat-4">category1</option>
                    <option value="cat-5">category1</option>
                    <option value="cat-6">category1</option>
                    <option value="cat-7">category1</option>
                    <option value="cat-8">category1</option>
                    <option value="cat-9">category1</option>
                    <option value="cat-10">category1</option>
                </select>
                <input className="search_bar" type="text" size="55" placeholder="search product..." />
                <button className="search_btn"> <FaSearch /> </button>
                <button className="cart"> <FaShoppingCart />  Cart</button>
                <button className="userSigning"><FaUserCircle /> Sign up/Sign in</button>

        </div>
    )
}

export default Navbar;
