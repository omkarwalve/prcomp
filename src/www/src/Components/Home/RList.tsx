import React, { useEffect, useState } from "react";
import Product from "Components/Listings/cogs/product";
import Card from "Components/Listings/Components/Card/Card";
import mock_json from './iphone.json';

const RECOMMENDER_URI = "http://localhost:8051/u/";

function ftimeout(ms: number) {
     let controller = new AbortController();
     setTimeout(() => controller.abort(), ms * 1000)
     return controller;
}

async function sleep(ms: number) { 
    return new Promise((resolve)  => setTimeout(resolve,ms * 1000));
}

async function recommend(uid: number): Promise<Product[]|undefined> {
	const reqOpts: RequestInit =  {
		method: 'GET',
		credentials: 'omit',
		headers: { 'Content-Type': 'application/json' },
		redirect: 'follow',
		signal: ftimeout(60).signal
	}

	try {
		let response = await fetch(`${RECOMMENDER_URI}/${uid}`, reqOpts)
							.then( rsp => { console.log(rsp); return rsp.json(); });
		if (response?.listings) {
			return Product.from(response);
		} else { return [] }
	} catch {
		console.error("Gadbad hogayi bhai");
	}
} 

async function mock_recommend(uid: number): Promise<Product[]|undefined> {
	return await sleep(2).then(() => Product.from(mock_json));
}

function MiniListings({uid}:{uid: number}) {
	const [recommended, setRecommended] = useState<Product[]|undefined>([]);
	useEffect(() => { 
		async function worker() {
			await mock_recommend(uid).then((pdx) => { console.log(pdx); setRecommended(pdx);}) 
		}
		worker()
	}, [uid]);
	return (
		<div className="product-section">
		    {
              recommended && recommended.map(product => {
                return ( <Card key={product.id} product={product} layout={"compact"} /> )
              })
			}
		</div>
	)
}

export default MiniListings;
