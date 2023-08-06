# JourneyDetailsPeriodTicketSuggestionsResultApiModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_error** | Option<**bool**> | Flag indicating that an error occurred while getting ticket suggestions. | [optional]
**ticket_suggestions** | Option<[**Vec<crate::models::JourneyDetailsPeriodTicketSuggestionApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.TicketSuggestionApiModel.md)> | Ticket suggestions for a journey. | [optional]
**ticket_validities** | Option<[**Vec<crate::models::JourneyDetailsPeriodTicketValidityApiModel>**](VT.ApiPlaneraResa.Web.V4.Models.JourneyDetails.TicketValidityApiModel.md)> | An array with the tickets from the existingTickets-array in the post-body. Validity for the journey is indicated for each ticket in the array. Included if 'ticketsuggestions' is passed in the includes array in the request, otherwise null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


