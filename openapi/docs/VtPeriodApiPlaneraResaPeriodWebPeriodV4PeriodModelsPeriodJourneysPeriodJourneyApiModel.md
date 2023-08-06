# JourneysPeriodJourneyApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reconstruction_reference** | Option<**String**> | A reference that can be used to reconstruct this journey at a later time. | [optional]
**details_reference** | Option<**String**> | A reference that should be used when getting detailed information about the journey. | [optional]
**departure_access_link** | Option<[**crate::models::JourneysPeriodDepartureAccessLinkApiModel**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.DepartureAccessLinkApiModel.md)> |  | [optional]
**trip_legs** | Option<[**Vec<crate::models::JourneysPeriodTripLegApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.TripLegApiModel.md)> | A list of trip legs on a journey, when applicable. A journey has either one or more trip legs or one  destination link. | [optional]
**connection_links** | Option<[**Vec<crate::models::JourneysPeriodConnectionLinkApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.ConnectionLinkApiModel.md)> | A list of ConnectionLinks between TripLegs, when applicable. The internal order of TripLegs and ConnectionLinks is defined by Index-property on the objects. | [optional]
**arrival_access_link** | Option<[**crate::models::JourneysPeriodArrivalAccessLinkApiModel**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.ArrivalAccessLinkApiModel.md)> |  | [optional]
**destination_link** | Option<[**crate::models::JourneysPeriodDestinationLinkApiModel**](VT.ApiPlaneraResa.Web.V4.Models.Journeys.DestinationLinkApiModel.md)> |  | [optional]
**is_departed** | Option<**bool**> | Flag indicating if the first trip leg of the journey is departed. | [optional]
**occupancy** | Option<[**crate::models::OccupancyInformationApiModel**](VT.ApiPlaneraResa.Web.V4.Models.OccupancyInformationApiModel.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


