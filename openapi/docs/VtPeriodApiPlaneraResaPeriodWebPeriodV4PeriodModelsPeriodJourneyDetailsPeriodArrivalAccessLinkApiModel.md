# JourneyDetailsPeriodArrivalAccessLinkApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transport_mode** | Option<[**crate::models::TransportMode**](VT.ApiPlaneraResa.Core.Models.TransportMode.md)> |  | [optional]
**transport_sub_mode** | Option<[**crate::models::TransportSubMode**](VT.ApiPlaneraResa.Core.Models.TransportSubMode.md)> |  | [optional]
**origin** | Option<[**crate::models::JourneyDetailsPeriodCallApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.CallApiModel.md)> |  | [optional]
**destination** | Option<[**crate::models::JourneyDetailsPeriodLinkEndpointApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.LinkEndpointApiModel.md)> |  | [optional]
**notes** | Option<[**Vec<crate::models::Note>**](VT.ApiPlaneraResa.Core.Models.Note.md)> | An ordered list (most important first) of notes related to the access link. | [optional]
**distance_in_meters** | Option<**i32**> | Distance in meters. | [optional]
**planned_departure_time** | Option<**String**> | The planned departure time in RFC 3339 format. | [optional]
**planned_arrival_time** | Option<**String**> | The planned arrival time in RFC 3339 format. | [optional]
**planned_duration_in_minutes** | Option<**i32**> | The planned duration in minutes. | [optional]
**estimated_departure_time** | Option<**String**> | The estimated departure time in RFC 3339 format, if available. | [optional]
**estimated_arrival_time** | Option<**String**> | The estimated arrival time in RFC 3339 format, if available. | [optional]
**estimated_duration_in_minutes** | Option<**i32**> | The estimated duration in minutes, if available. | [optional]
**estimated_number_of_steps** | Option<**i32**> | Number of steps based on the distance and an estimated step length of 0.65 meters. | [optional]
**link_coordinates** | Option<[**Vec<crate::models::CoordinateApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.CoordinateApiModel.md)> | The coordinates for the link. | [optional]
**segments** | Option<[**Vec<crate::models::JourneyDetailsPeriodLinkSegmentApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.LinkSegmentApiModel.md)> | The segments that make up this link. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


