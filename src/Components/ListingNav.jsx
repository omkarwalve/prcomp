import React, { useState } from 'react';
import './ListingNav.css';
import { FaSearch } from "react-icons/fa";
import { FaShoppingCart } from "react-icons/fa";
import { FaUserCircle } from "react-icons/fa";
import { GrCart } from "react-icons/gr";
import { GiHamburgerMenu } from "react-icons/gi"; 
import { Link } from 'react-router-dom';
import Footer from './Footer';
import { BrowserRouter as Router, Route} from 'react-router-dom';

export default class ListingNav extends React.Component {

    render() {
        return (
            <div className="navBar">

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
                <Router>
                <Link>
                <button className="cart" component ={Footer} to exact ="/footer" > <GrCart /></button>
                </Link>
                </Router>
                <button className="menu"><GiHamburgerMenu /></button>

            </div>


        )
    }
}
