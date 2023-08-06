# \StopAreasApi

All URIs are relative to *https://ext-api.vasttrafik.se/pr/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stop_areas_stop_area_gid_arrivals_details_reference_details_get**](StopAreasApi.md#stop_areas_stop_area_gid_arrivals_details_reference_details_get) | **GET** /stop-areas/{stopAreaGid}/arrivals/{detailsReference}/details | Returns details about an arrival.
[**stop_areas_stop_area_gid_arrivals_get**](StopAreasApi.md#stop_areas_stop_area_gid_arrivals_get) | **GET** /stop-areas/{stopAreaGid}/arrivals | Returns arrivals to the specified stop area at the specified time.
[**stop_areas_stop_area_gid_departures_details_reference_details_get**](StopAreasApi.md#stop_areas_stop_area_gid_departures_details_reference_details_get) | **GET** /stop-areas/{stopAreaGid}/departures/{detailsReference}/details | Returns details about a departure.
[**stop_areas_stop_area_gid_departures_get**](StopAreasApi.md#stop_areas_stop_area_gid_departures_get) | **GET** /stop-areas/{stopAreaGid}/departures | Returns departures from the specified stop area at the specified time.



## stop_areas_stop_area_gid_arrivals_details_reference_details_get

> crate::models::DeparturesAndArrivalsPeriodArrivalDetailsApiModel stop_areas_stop_area_gid_arrivals_details_reference_details_get(details_reference, stop_area_gid, includes)
Returns details about an arrival.

Sample request:        GET /stop-areas/9021014001760000/arrivals/{detailsReference}/details?includes=servicejourneycalls

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**details_reference** | **String** | The reference to the service journey, received from the arrivals call. A detailsReference is only valid during the same day as it was generated. | [required] |
**stop_area_gid** | **String** |  | [required] |
**includes** | Option<[**Vec<crate::models::ArrivalDetailsIncludeType>**](crate::models::ArrivalDetailsIncludeType.md)> | The additional information to include in the response. |  |

### Return type

[**crate::models::DeparturesAndArrivalsPeriodArrivalDetailsApiModel**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.ArrivalDetailsApiModel.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_areas_stop_area_gid_arrivals_get

> crate::models::DeparturesAndArrivalsPeriodGetArrivalsResponse stop_areas_stop_area_gid_arrivals_get(stop_area_gid, start_date_time, platforms, time_span_in_minutes, max_arrivals_per_line_and_direction, limit, offset)
Returns arrivals to the specified stop area at the specified time.

Sample request:        GET /stop-areas/9021014003980000/arrivals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stop_area_gid** | **String** | The 16-digit Västtrafik gid of the stop area. | [required] |
**start_date_time** | Option<**String**> | The start of the time interval for which to get upcoming arrivals. Must be specified in RFC 3339 format or be null which means that the current time on the server is used. |  |
**platforms** | Option<**String**> | Filter results by platform. Multiple platforms are separated by comma. Case sensitive. |  |
**time_span_in_minutes** | Option<**i32**> | The number of minutes from the start time for which to get upcoming arrivals. Allowed values are between 0 and 1440. |  |[default to 60]
**max_arrivals_per_line_and_direction** | Option<**i32**> | The maximum number of arrivals for a single line in a specific direction. |  |[default to 2]
**limit** | Option<**i32**> | The number of results to return. |  |[default to 10]
**offset** | Option<**i32**> | The zero-based start offset of the pagination. |  |[default to 0]

### Return type

[**crate::models::DeparturesAndArrivalsPeriodGetArrivalsResponse**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.GetArrivalsResponse.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_areas_stop_area_gid_departures_details_reference_details_get

> crate::models::DeparturesAndArrivalsPeriodDepartureDetailsApiModel stop_areas_stop_area_gid_departures_details_reference_details_get(details_reference, stop_area_gid, includes)
Returns details about a departure.

Sample request:        GET /stop-areas/9021014001760000/departures/{detailsReference}/details?includes=servicejourneycalls

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**details_reference** | **String** | The reference to the service journey, received from the departures call. A detailsReference is only valid during the same day as it was generated. | [required] |
**stop_area_gid** | **String** |  | [required] |
**includes** | Option<[**Vec<crate::models::DepartureDetailsIncludeType>**](crate::models::DepartureDetailsIncludeType.md)> | The additional information to include in the response. |  |

### Return type

[**crate::models::DeparturesAndArrivalsPeriodDepartureDetailsApiModel**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.DepartureDetailsApiModel.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_areas_stop_area_gid_departures_get

> crate::models::DeparturesAndArrivalsPeriodGetDeparturesResponse stop_areas_stop_area_gid_departures_get(stop_area_gid, start_date_time, platforms, time_span_in_minutes, max_departures_per_line_and_direction, limit, offset, include_occupancy)
Returns departures from the specified stop area at the specified time.

Sample request:        GET /stop-areas/9021014003980000/departures

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stop_area_gid** | **String** | The 16-digit Västtrafik gid of the stop area. | [required] |
**start_date_time** | Option<**String**> | The start of the time interval for which to get upcoming departures. Must be specified in RFC 3339 format or be null which means that the current time on the server is used. |  |
**platforms** | Option<**String**> | Filter results by platform. Multiple platforms are separated by comma. Case sensitive. |  |
**time_span_in_minutes** | Option<**i32**> | The number of minutes from the start time for which to get upcoming departures. Allowed values are between 0 and 1440. |  |[default to 60]
**max_departures_per_line_and_direction** | Option<**i32**> | The maximum number of departures for a single line in a specific direction. |  |[default to 2]
**limit** | Option<**i32**> | The number of results to return. |  |[default to 10]
**offset** | Option<**i32**> | The zero-based start offset of the pagination. |  |[default to 0]
**include_occupancy** | Option<**bool**> | Includes occupancy in departure. |  |[default to false]

### Return type

[**crate::models::DeparturesAndArrivalsPeriodGetDeparturesResponse**](VT.ApiPlaneraResa.Web.V4.Models.DeparturesAndArrivals.GetDeparturesResponse.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

