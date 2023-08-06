# \JourneysApi

All URIs are relative to *https://ext-api.vasttrafik.se/pr/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**journeys_details_reference_details_get**](JourneysApi.md#journeys_details_reference_details_get) | **GET** /journeys/{detailsReference}/details | Returns details about a journey.
[**journeys_get**](JourneysApi.md#journeys_get) | **GET** /journeys | Returns journeys matching the specified search parameters.
[**journeys_reconstruct_get**](JourneysApi.md#journeys_reconstruct_get) | **GET** /journeys/reconstruct | Reconstructs a journey based on the given reconstruction reference, received from the search journeys query.
[**journeys_valid_time_interval_get**](JourneysApi.md#journeys_valid_time_interval_get) | **GET** /journeys/valid-time-interval | Returns a time interval for when journey data is available.



## journeys_details_reference_details_get

> crate::models::JourneyDetailsPeriodJourneyDetailsApiModel journeys_details_reference_details_get(details_reference, includes, channel_ids, product_types, traveller_categories)
Returns details about a journey.

Sample request:        GET /journeys/{detailsReference}/details?includes=ticketsuggestions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**details_reference** | **String** | The reference to the journey, received from the search journeys query. A detailsReference is only valid during the same day as it was generated. | [required] |
**includes** | Option<[**Vec<crate::models::JourneyDetailsIncludeType>**](crate::models::JourneyDetailsIncludeType.md)> | The additional information to include in the response. |  |
**channel_ids** | Option<[**Vec<i32>**](i32.md)> | List of channel ids to include if 'ticketsuggestions' is set in the 'includes' parameter. Optional parameter. |  |
**product_types** | Option<[**Vec<i32>**](i32.md)> | List of product type ids to include if 'ticketsuggestions' is set in the 'includes' parameter. Optional parameter. |  |
**traveller_categories** | Option<[**Vec<crate::models::TravellerCategory>**](crate::models::TravellerCategory.md)> | List of traveller category ids to include if 'ticketsuggestions' is set in the 'includes' parameter. Optional parameter. |  |

### Return type

[**crate::models::JourneyDetailsPeriodJourneyDetailsApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.JourneyDetailsApiModel.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## journeys_get

> crate::models::JourneysPeriodGetJourneysResponse journeys_get(origin_gid, origin_name, origin_latitude, origin_longitude, destination_gid, destination_name, destination_latitude, destination_longitude, date_time, date_time_relates_to, pagination_reference, limit, transport_modes, transport_sub_modes, only_direct_connections, include_nearby_stop_areas, via_gid, origin_walk, dest_walk, origin_bike, dest_bike, total_bike, origin_car, dest_car, origin_park, dest_park, interchange_duration_in_minutes, include_occupancy)
Returns journeys matching the specified search parameters.

For an origin or destination to be valid, either a gid or a combination of latitude and longitude must be specified. OriginName and destinationName are optional in combination with latitude and longitude.    Sample request:        GET /journeys?originGid=9021014001760000&destinationGid=9021014003980000    or        GET /journeys?originName=Sadelsten,+V%C3%A5rg%C3%A5rda&originLongitude=12.63308&originLatitude=58.028237&destinationLongitude=11.930897&destinationLatitude=57.586085&destinationName=%C3%85sdammsstigen,+427+36+Billdal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**origin_gid** | Option<**String**> | The 16-digit Västtrafik gid of the origin location (which could be either a stop area (e.g. '9021014001760000'), a stop point (e.g. '9022014001760004') or meta-station (e.g. '0000000800000022')). |  |
**origin_name** | Option<**String**> | The name of the origin location. The maximum length allowed is 256 characters. |  |
**origin_latitude** | Option<**f64**> | The latitude of the origin location. |  |
**origin_longitude** | Option<**f64**> | The longitude of the origin location. |  |
**destination_gid** | Option<**String**> | The 16-digit Västtrafik gid of the destination location (which could be either a stop area, stop point or meta-station). |  |
**destination_name** | Option<**String**> | The name of the destination location. The maximum length allowed is 256 characters. |  |
**destination_latitude** | Option<**f64**> | The latitude of the destination location. |  |
**destination_longitude** | Option<**f64**> | The longitude of the destination location. |  |
**date_time** | Option<**String**> | The datetime for which to search journeys. Must be specified in RFC 3339 format or be null which means that the current time on the server is used. The related dateTimeRelatesTo parameter specifies if the time is related to the arrival or departure. |  |
**date_time_relates_to** | Option<[**DateTimeRelatesToType**](.md)> | Specifies if the datetime is related to the departure or arrival of the journey. |  |
**pagination_reference** | Option<**String**> | Pagination reference from a previous search. |  |
**limit** | Option<**i32**> | The number of results to return. Not guaranteed to return the specified number of results and usually not more than 7 results. |  |[default to 10]
**transport_modes** | Option<[**Vec<crate::models::JourneyTransportMode>**](crate::models::JourneyTransportMode.md)> | The transport modes to include when searching for journeys, if none specified all transport modes are included. |  |
**transport_sub_modes** | Option<[**Vec<crate::models::JourneyTransportSubMode>**](crate::models::JourneyTransportSubMode.md)> | The transport sub modes to include when searching for journeys, if none specified all transport sub modes are included. Only supported in combination with transportMode 'train'. |  |
**only_direct_connections** | Option<**bool**> | Only include direct connections, e.g. journeys with one trip leg. |  |[default to false]
**include_nearby_stop_areas** | Option<**bool**> | Includes nearby stop areas when searching for a journey to or from a stop area or stop point. This means that the search algorithm will take additional stop points of other stop areas nearby the given start and destination stop area into account. These additional stop points are reachable by walk. E.g when true a journey suggestion may include a departure access link (initial walking leg) to a stop point of a stop area close by the specified origin stop area. |  |[default to false]
**via_gid** | Option<**i64**> | The 16-digit Västtrafik gid of the via location (which must be a stop area). |  |
**origin_walk** | Option<**String**> | Enables/disables using footpaths in the beginning of a trip when searching from an address. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable walk with a minimum distance of 0 meters and a maximum distance of 3 kilometers, set the parameter originWalk=1,0,3000. If default distances should be used, skip the values, e.g 1,,. This will enable walk with the default minimum (0 meters) and the default maximum (2000 meters). |  |
**dest_walk** | Option<**String**> | Enables/disables using footpaths at the end of a trip when searching to an address. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable walk with a minimum distance of 0 meters and a maximum distance of 3 kilometers, set the parameter destWalk=1,0,3000. If default distances should be used, skip the values, e.g 1,,. This will enable walk with the default minimum (0 meters) and the default maximum (2000 meters). |  |
**origin_bike** | Option<**String**> | Enables/disables using bike paths in the beginning of a trip when searching from an address. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable bike with a minimum distance of 1000 meters and a maximum distance of 5 kilometers, set the parameter originBike=1,1000,5000. If default distances should be used, skip the values, e.g 1,,. This will enable bike with the default minimum (0 meters) and the default maximum (3000 meters). |  |
**dest_bike** | Option<**String**> | Enables/disables using bike paths at the end of a trip when searching to an address. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable bike with a minimum distance of 1000 meters and a maximum distance of 5 kilometers, set the parameter destBike=1,1000,5000. If default distances should be used, skip the values, e.g 1,,. This will enable bike with the default minimum (0 meters) and the default maximum (3000 meters). |  |
**total_bike** | Option<**String**> | Enables/disables using bike routes for the whole trip. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable bike with a minimum distance of 0 meters and a maximum distance of 20 kilometers, set the parameter totalBike=1,0,20000. If default distances should be used, skip the values, e.g 1,,. This will enable bike with the default minimum (0 meters) and the default maximum (25000 meters). |  |
**origin_car** | Option<**String**> | Enables/disables using car in the beginning of a trip when searching from an address. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable car with a minimum distance of 2000 meters and a maximum distance of 7 kilometers, set the parameter origincar=1,2000,7000. If default distances should be used, skip the values, e.g 1,,. This will enable car with the default minimum (0 meters) and the default maximum (5000 meters). |  |
**dest_car** | Option<**String**> | Enables/disables using car at the end of a trip when searching to an address. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable car with a minimum distance of 2000 meters and a maximum distance of 7 kilometers, set the parameter destCar=1,2000,7000. If default distances should be used, skip the values, e.g 1,,. This will enable car with the default minimum (0 meters) and the default maximum (5000 meters). |  |
**origin_park** | Option<**String**> | Enables/disables using Park and Ride in the beginning of a trip when searching from an address. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable Park and Ride with a minimum distance of 3000 meters and a maximum distance of 70 kilometers, set the parameter originPark=1,3000,70000. If default distances should be used, skip the values, e.g 1,,. This will enable Park and Ride with the default minimum (2000 meters) and the default maximum (50000 meters). |  |
**dest_park** | Option<**String**> | Enables/disables using Park and Ride at the end of a trip when searching to an address. To fine-tune the minimum and/or maximum distance to the next public transport station, provide these values separated by comma. The values are expressed in meters. To enable Park and Ride with a minimum distance of 3000 meters and a maximum distance of 70 kilometers, set the parameter destPark=1,3000,70000. If default distances should be used, skip the values, e.g 1,,. This will enable Park and Ride with the default minimum (2000 meters) and the default maximum (50000 meters). |  |
**interchange_duration_in_minutes** | Option<**i32**> | The minimum number of minutes between arrival and departure for a connection to be valid and the trip included in the search results, ignoring the default value. |  |
**include_occupancy** | Option<**bool**> | Includes occupancy in journey. |  |[default to false]

### Return type

[**crate::models::JourneysPeriodGetJourneysResponse**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.GetJourneysResponse.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## journeys_reconstruct_get

> crate::models::JourneysPeriodJourneyApiModel journeys_reconstruct_get(r#ref, include_occupancy)
Reconstructs a journey based on the given reconstruction reference, received from the search journeys query.

Sample request:        GET /journeys/reconstruct?ref=¶HKI¶T$A=1@O=Brunnsparken, Göteborg@L=1760003@a=128@$A=1@O=Korsvägen, Göteborg@L=3980004@a=128@$202206131358$202206131406$Spå    4$$1$$$$$$¶KRCC¶#VE#1#

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#ref** | **String** | The reconstruction reference. A reconstructionReference is valid as long as the original journey search is valid. | [required] |
**include_occupancy** | Option<**bool**> | Includes occupancy in journey. |  |[default to false]

### Return type

[**crate::models::JourneysPeriodJourneyApiModel**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.JourneyApiModel.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## journeys_valid_time_interval_get

> crate::models::ValidTimeIntervalApiModel journeys_valid_time_interval_get()
Returns a time interval for when journey data is available.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ValidTimeIntervalApiModel**](VT.ApiPlaneraResa.Web.V4.Models.ValidTimeIntervalApiModel.md)

### Authorization

[auth](../README.md#auth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

