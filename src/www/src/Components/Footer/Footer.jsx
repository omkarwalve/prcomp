//   _______  _______  _______  _______  _______  ______   
//  |       ||       ||       ||       ||       ||    _ |  
//  |    ___||   _   ||   _   ||_     _||    ___||   | ||  
//  |   |___ |  | |  ||  | |  |  |   |  |   |___ |   |_||_ 
//  |    ___||  |_|  ||  |_|  |  |   |  |    ___||    __  |
//  |   |    |       ||       |  |   |  |   |___ |   |  | |
//  |___|    |_______||_______|  |___|  |_______||___|  |_|

// Library Imports
import React,{ useState } from 'react';

// File Imports
import Select from 'Components/Assets/Select/Select';
import { ReactComponent as Kilowog } from 'assets/kilowog.svg';
import { ReactComponent as Globe } from './assets/globe.svg';

// CSS
import './Footer.css';

const Block = ({items}) => {
    return(
        <ul className="service-block">
            {
                items.map( item => {
                    var liObj = Object.values(item);
                    return (
                        <li key={ liObj[0].toLowerCase() } className="service-item">
                            <a href={liObj[1]}>{liObj[0]}</a>
                        </li>
                )})
            }
        </ul>
    )
}

const Footer = () => {

    const customerOptions = [
        {
            label: 'Contact Us',
            href : '#'
        },
        {
            label: 'Help',
            href : '#'
        },
        {
            label: 'Feedback',
            href : '#'
        },
        {
            label: 'About Us',
            href : '#'
        },
    ];
    const businessOptions = [
        {
            label: 'Setup Your Store',
            href : '#'
        },
        {
            label: 'Affiliate Plans',
            href : '#'
        },
        {
            label: 'Using our API',
            href : '#'
        },
        {
            label: 'Have an issue?',
            href : '#'
        },
    ];
    const languages = [
        {
            label: 'English',
            langcode: 'en'
        },
        {
            label: 'हिंदी',
            langcode: 'hi'
        },
    ];
    const regions = [
        {
            label: 'India',
            regioncode: 'in'
        },
        {
            label: 'United Kingdom',
            regioncode: 'uk'
        },
        {
            label: 'USA',
            regioncode: 'us'
        },
        {
            label: 'Germany',
            regioncode: 'gr'
        },
    ];
    const [activeRegion, setRegion] = useState(null);
    const [activeLang, setLang] = useState(null);

    return (
        <footer className="footer-section">
            <div className="footer-content">
                <Kilowog />
                <span className="region-selection">
                    <Globe />
                    <Select items={regions} optionHolder={setRegion} position="top" />
                </span>
                <Select items={languages} optionHolder={setLang} position="top" />
                <Block items={customerOptions}/>
                <Block items={businessOptions}/>
            </div>
        </footer>
    )
}

export default Footer
