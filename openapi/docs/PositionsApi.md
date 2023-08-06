# \PositionsApi

All URIs are relative to *https://ext-api.vasttrafik.se/pr/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**positions_get**](PositionsApi.md#positions_get) | **GET** /positions | Returns journey positions within a bounding box



## positions_get

> Vec<crate::models::PositionsPeriodJourneyPositionApiModel> positions_get(lower_left_lat, lower_left_long, upper_right_lat, upper_right_long, transport_modes, details_references, line_designations, limit)
Returns journey positions within a bounding box

Sample request:        GET /positions?lowerLeftLat=57.721723&lowerLeftLong=12.011882&upperRightLat=57.737549&upperRightLong=12.039268&limit=100

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lower_left_lat** | **f64** | Lower left latitude of bounding box. | [required] |
**lower_left_long** | **f64** | Lower left longitude of bounding box. | [required] |
**upper_right_lat** | **f64** | Upper right latitude of bounding box. | [required] |
**upper_right_long** | **f64** | Upper right longitude of bounding box. | [required] |
**transport_modes** | Option<[**Vec<crate::models::PositionTransportMode>**](crate::models::PositionTransportMode.md)> | The transport modes to include when searching for journeys, if none specified all transport modes are included. |  |
**details_references** | Option<[**Vec<String>**](String.md)> | Filter journeys by one or more journey details reference. |  |
**line_designations** | Option<[**Vec<String>**](String.md)> | Only journeys running the given lineDesignations (case sensitive) are part of the result. |  |
**limit** | Option<**i32**> | Maximum number of journeys in response. Range from 1 to 200. Defaults to 100 |  |[default to 100]

### Return type

[**Vec<crate::models::PositionsPeriodJourneyPositionApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.Positions.JourneyPositionApiModel.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

