# Nabu's Algorithm

## CORE
#### - ~~Synchronus~~ [Now Redundant]

1. Take `QUERY` and `CATEGORY` as input.
2. Loop through the `CATEGORY` file for the list of sites to be scraped. 
> File should look like 
> ```
> Amazon
> Flipkart
> Alibaba
> ```
3. Fetch the object file(`PROFILE`) for a website and...
    1. Stage 1 Scraping
        - Make HTTP/API request to the query page with the `QUERY`.
        - Collect all listing urls into `LIST`.
    2. Stage 2 Scraping
        - Make HTTP request to element of `LIST`.
            - Parse DOM tree and scrape out data from elements pointed by `PROFILE`, append in `JSON` format.
        - Proceed to next element in `LIST`.
4. Return the complete `JSON` as output.

#### - Asynchronus + Threading

1. Take `QUERY` and `CATEGORY` as input.
2. Loop through the `CATEGORY` file for the list of sites to be scraped. 
> File should look like 
> ```
> Amazon
> Flipkart
> Alibaba
> ```
3. Fetch the object file(`PROFILE`) for a website and start a worker thread for it...
    1. *Stage 1* Scraping
        - Make HTTP/API request to the query page with the `QUERY`.
        - Parse the html and extract into `LISTING` data structure.
    2. *Stage 2* Scraping
        - Make HTTP concurrent http requests asynchronuosly to `url` object of `LISTING`.
            - Parse DOM tree and scrape out data from elements pointed by `PROFILE`, append into `JSON` format.
        - Proceed to next `LISTING`.
4. Return the complete `JSON` as output.

## Stage 1

1. Make HTTP request to each website obtained from `CATEGORY`.
2. Parse the HTML file.
3. Lookup *Stage 1* elements in the HTML specified in `PROFILE` and generate a list of urls.
4. Pass the list to *Stage 2*.

## Stage 2

1. Make concurrent HTTP request to each url in a `LISTING` object.
2. Await responses for all connections.
3. Upon recieving response parse the html and extract the data wref to `PROFILE`.
4. Update the `LISTING` data struct with the scraped data.
5. Return `LISTING`.
