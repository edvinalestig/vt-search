# \LocationsApi

All URIs are relative to *https://ext-api.vasttrafik.se/pr/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**locations_by_coordinates_get**](LocationsApi.md#locations_by_coordinates_get) | **GET** /locations/by-coordinates | Returns the locations nearest the specified coordinates. Currently only stop areas, stop points and meta-stations are supported.
[**locations_by_text_get**](LocationsApi.md#locations_by_text_get) | **GET** /locations/by-text | Returns locations matching the specified text. Currently only stop areas, addresses, points of interest and meta-stations are supported.



## locations_by_coordinates_get

> crate::models::LocationsPeriodGetLocationsResponse locations_by_coordinates_get(latitude, longitude, radius_in_meters, types, limit, offset)
Returns the locations nearest the specified coordinates. Currently only stop areas, stop points and meta-stations are supported.

Sample request:        GET /locations/by-coordinates?latitude=57.708734&longitude=11.974764&radiusInMeters=500&limit=10&offset=0

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**latitude** | **f64** | The latitude. | [required] |
**longitude** | **f64** | The longitude. | [required] |
**radius_in_meters** | Option<**i32**> | The search radius from the coordinates specified in meters. Must be a positive integer > 0. |  |[default to 500]
**types** | Option<[**Vec<crate::models::LocationByCoordinatesType>**](crate::models::LocationByCoordinatesType.md)> | The location types to include in the response, if none specified all locations types are included. |  |
**limit** | Option<**i32**> | The number of results to return. |  |[default to 10]
**offset** | Option<**i32**> | The zero-based start offset of the pagination. |  |[default to 0]

### Return type

[**crate::models::LocationsPeriodGetLocationsResponse**](VT.ApiPlaneraResa.Web.V4.Models.Locations.GetLocationsResponse.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locations_by_text_get

> crate::models::LocationsPeriodGetLocationsResponse locations_by_text_get(q, types, limit, offset)
Returns locations matching the specified text. Currently only stop areas, addresses, points of interest and meta-stations are supported.

Sample request:        GET /locations/by-text?q=brunnsparken&limit=10&offset=0

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | The search text (e.g. 'brunn', 'cent' or 'Kungsgatan'). The maximum length allowed is 256 characters. | [required] |
**types** | Option<[**Vec<crate::models::LocationByTextType>**](crate::models::LocationByTextType.md)> | The location types to include in the response, if none specified all locations types are included. |  |
**limit** | Option<**i32**> | The number of results to return. |  |[default to 10]
**offset** | Option<**i32**> | The zero-based start offset of the pagination. |  |[default to 0]

### Return type

[**crate::models::LocationsPeriodGetLocationsResponse**](VT.ApiPlaneraResa.Web.V4.Models.Locations.GetLocationsResponse.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

