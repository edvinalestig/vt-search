# JourneyDetailsPeriodJourneyDetailsApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**departure_access_link** | Option<[**crate::models::JourneyDetailsPeriodDepartureAccessLinkApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.DepartureAccessLinkApiModel.md)> |  | [optional]
**trip_legs** | Option<[**Vec<crate::models::JourneyDetailsPeriodTripLegDetailsApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.TripLegDetailsApiModel.md)> | Detailed information, including stops, about the trip legs. | [optional]
**connection_links** | Option<[**Vec<crate::models::JourneyDetailsPeriodConnectionLinkApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.ConnectionLinkApiModel.md)> | A list of ConnectionLinks between TripLegs, when applicable. The internal order of TripLegs and ConnectionLinks is defined by Index-property on the objects. | [optional]
**arrival_access_link** | Option<[**crate::models::JourneyDetailsPeriodArrivalAccessLinkApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.ArrivalAccessLinkApiModel.md)> |  | [optional]
**destination_link** | Option<[**crate::models::JourneyDetailsPeriodDestinationLinkApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.DestinationLinkApiModel.md)> |  | [optional]
**ticket_suggestions_result** | Option<[**crate::models::JourneyDetailsPeriodTicketSuggestionsResultApiModel**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.TicketSuggestionsResultApiModel.md)> |  | [optional]
**tariff_zones** | Option<[**Vec<crate::models::JourneyDetailsPeriodTariffZoneApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.TariffZoneApiModel.md)> | The tariff zones that the journey traverses. | [optional]
**occupancy** | Option<[**crate::models::OccupancyInformationApiModel**](VT.ApiPlaneraResa.Web.V4.Models.OccupancyInformationApiModel.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


